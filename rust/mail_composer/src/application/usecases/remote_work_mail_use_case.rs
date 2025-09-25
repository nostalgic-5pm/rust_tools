use crate::domain::{
    entities::mail_draft::MailDraft,
    interfaces::{
        address_book::AddressBookPort, configuration::ConfigurationPort,
        mail_client::MailClientPort, mail_config::MailConfigPort, work_time::WorkTimePort,
    },
    value_objects::{
        email_address::EmailAddress,
        mail_objects::{MailBody, Subject, WorkTime, WorkTimeRange},
    },
};
use share::error::app_error::AppResult;

/// 在宅勤務メール作成のユースケース
pub struct RemoteWorkMailUseCase<A, C, M, W, MC>
where
    A: AddressBookPort,
    C: ConfigurationPort,
    M: MailClientPort,
    W: WorkTimePort,
    MC: MailConfigPort,
{
    address_book_port: A,
    configuration_port: C,
    mail_client_port: M,
    work_time_port: W,
    mail_config_port: MC,
}

impl<A, C, M, W, MC> RemoteWorkMailUseCase<A, C, M, W, MC>
where
    A: AddressBookPort,
    C: ConfigurationPort,
    M: MailClientPort,
    W: WorkTimePort,
    MC: MailConfigPort,
{
    /// 新しいRemoteWorkMailUseCaseを作成する
    pub fn new(
        address_book_port: A,
        configuration_port: C,
        mail_client_port: M,
        work_time_port: W,
        mail_config_port: MC,
    ) -> Self {
        Self {
            address_book_port,
            configuration_port,
            mail_client_port,
            work_time_port,
            mail_config_port,
        }
    }

    /// 名前のリストからメールアドレスのリストを解決する
    fn resolve_email_addresses(&self, names: &[&str]) -> AppResult<Vec<EmailAddress>> {
        self.address_book_port.resolve_many(names)
    }

    /// 在宅勤務開始メールを作成・送信する
    ///
    /// ## Arguments
    /// * `is_dry_run` - ドライランモード
    ///
    /// ## Returns
    /// * 成功時 - `Ok(())`
    /// * 失敗時 - `Err<AppError>`
    pub fn send_remote_work_start(&self, is_dry_run: bool) -> AppResult<()> {
        let config = self.configuration_port.load_configuration()?;
        let mail_config = self.mail_config_port.load_mail_config()?;

        // 在宅勤務開始設定を取得
        let start_config = mail_config
            .get_mail_type("remote_work_start")
            .ok_or_else(|| {
                share::error::app_error::AppError::new(share::error::kind::ErrorKind::NotFound)
                    .with_message("remote_work_start 設定が見つかりません")
            })?;

        // 現在時刻を取得
        let now_time = WorkTime::now()?;

        // 作業開始時刻を保存
        self.work_time_port.save_today_start_time(&now_time)?;

        // メールアドレスを解決
        let to_names: Vec<&str> = start_config.to_names.iter().map(|s| s.as_str()).collect();
        let cc_names: Vec<&str> = start_config.cc_names.iter().map(|s| s.as_str()).collect();
        let to_addresses = self.resolve_email_addresses(&to_names)?;
        let cc_addresses = self.resolve_email_addresses(&cc_names)?;

        // 件名と本文をテンプレートから生成
        let subject = Subject::new(start_config.format_subject(
            &config.department,
            &config.from,
            now_time.as_str(),
        ))?;

        let body = MailBody::new(&start_config.format_body(None));

        // メールドラフトを作成
        let draft = MailDraft::new(to_addresses, cc_addresses, subject, body);
        // メール送信/ドライラン
        self.mail_client_port.compose_mail(&draft, is_dry_run)
    }

    /// 在宅勤務終了メールを作成・送信する
    ///
    /// ## Arguments
    /// * `is_dry_run` - ドライランモード
    ///
    /// ## Returns
    /// * 成功時 - `Ok(())`
    /// * 失敗時 - `Err<AppError>`
    pub fn send_remote_work_end(&self, is_dry_run: bool) -> AppResult<()> {
        let config = self.configuration_port.load_configuration()?;
        let mail_config = self.mail_config_port.load_mail_config()?;

        // 在宅勤務終了設定を取得
        let end_config = mail_config
            .get_mail_type("remote_work_end")
            .ok_or_else(|| {
                share::error::app_error::AppError::new(share::error::kind::ErrorKind::NotFound)
                    .with_message("remote_work_end 設定が見つかりません")
            })?;

        // 現在時刻を取得
        let end_time = WorkTime::now()?;

        // 今日の開始時刻を読み込み
        let start_time = self
            .work_time_port
            .load_today_start_time()?
            .unwrap_or_else(|| WorkTime::new("--:--").unwrap());

        // メールアドレスを解決
        let to_names: Vec<&str> = end_config.to_names.iter().map(|s| s.as_str()).collect();
        let cc_names: Vec<&str> = end_config.cc_names.iter().map(|s| s.as_str()).collect();
        let to_addresses = self.resolve_email_addresses(&to_names)?;
        let cc_addresses = self.resolve_email_addresses(&cc_names)?;

        // 作業時間範囲を作成
        let work_range = WorkTimeRange::new(start_time, end_time.clone());

        // 件名と本文をテンプレートから生成
        let subject = Subject::new(end_config.format_subject(
            &config.department,
            &config.from,
            end_time.as_str(),
        ))?;

        let body = MailBody::new(&end_config.format_body(Some(&work_range.to_string())));

        // メールドラフトを作成
        let draft = MailDraft::new(to_addresses, cc_addresses, subject, body);

        // メール送信/ドライラン
        self.mail_client_port.compose_mail(&draft, is_dry_run)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::outbound::{
        json_address_book_adapter::JsonAddressBookAdapter,
        json_configuration_adapter::JsonConfigurationAdapter,
        json_mail_config_adapter::JsonMailConfigAdapter,
        json_work_time_adapter::JsonWorkTimeAdapter,
        thunderbird_mail_client_adapter::ThunderbirdMailClientAdapter,
    };

    #[test]
    fn test_remote_work_start_dry_run() {
        let address_book = JsonAddressBookAdapter::load_from_address_book(std::path::Path::new(
            "rust/mail_composer/config/address_book.json",
        ))
        .unwrap();
        let config = JsonConfigurationAdapter::with_default_path();
        let mail_client = ThunderbirdMailClientAdapter::new("thunderbird");
        let work_time = JsonWorkTimeAdapter::with_default_settings();
        let mail_config = JsonMailConfigAdapter::new();

        let use_case =
            RemoteWorkMailUseCase::new(address_book, config, mail_client, work_time, mail_config);

        // ドライランでテスト
        let result = use_case.send_remote_work_start(true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_remote_work_end_dry_run() {
        let address_book = JsonAddressBookAdapter::load_from_address_book(std::path::Path::new(
            "rust/mail_composer/config/address_book.json",
        ))
        .unwrap();
        let config = JsonConfigurationAdapter::with_default_path();
        let mail_client = ThunderbirdMailClientAdapter::new("thunderbird");
        let work_time = JsonWorkTimeAdapter::with_default_settings();
        let mail_config = JsonMailConfigAdapter::new();

        // 事前に開始時間を設定
        let start_time = WorkTime::new("09:00").unwrap();
        work_time.save_today_start_time(&start_time).unwrap();

        let use_case =
            RemoteWorkMailUseCase::new(address_book, config, mail_client, work_time, mail_config);

        let result = use_case.send_remote_work_end(true);
        match &result {
            Ok(_) => println!("✅ Remote work end test passed!"),
            Err(e) => println!("❌ Remote work end test failed: {}", e),
        }
        assert!(result.is_ok());
    }
}
