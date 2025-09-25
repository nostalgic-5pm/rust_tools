use crate::domain::{
    entities::mail_draft::MailDraft,
    interfaces::mail_client::MailClientPort,
};
use share::{
    error::{
        app_error::{AppError, AppResult},
        kind::ErrorKind,
    },
};
use std::process::Command;

/// Thunderbirdメールクライアントのアウトバウンドアダプター
pub struct ThunderbirdMailClientAdapter {
    thunderbird_exe_path: String,
}

impl ThunderbirdMailClientAdapter {
    /// 新しいThunderbirdMailClientAdapterを作成する
    ///
    /// ## Arguments
    /// * `thunderbird_exe_path` - Thunderbird実行ファイルのパス
    ///
    /// ## Returns
    /// * ThunderbirdMailClientAdapterのインスタンス
    pub fn new(thunderbird_exe_path: impl Into<String>) -> Self {
        Self {
            thunderbird_exe_path: thunderbird_exe_path.into(),
        }
    }

    /// Thunderbird compose引数を構築する
    fn build_compose_arg(&self, draft: &MailDraft) -> String {
        let to = draft.to_addresses_as_string();
        let cc = draft.cc_addresses_as_string();
        let subject = draft.subject().as_str();
        let body = draft.body().to_crlf();

        // 必要に応じてエスケープ処理
        let escape_quotes = |s: &str| s.replace('\'', "'");

        format!(
            "format=plain,to='{}',cc='{}',subject='{}',body='{}'",
            escape_quotes(&to),
            escape_quotes(&cc),
            escape_quotes(subject),
            escape_quotes(&body),
        )
    }
}

impl MailClientPort for ThunderbirdMailClientAdapter {
    fn compose_mail(&self, draft: &MailDraft, is_dry_run: bool) -> AppResult<()> {
        let compose_arg = self.build_compose_arg(draft);

        if is_dry_run {
            println!("[DRY-RUN] {} -compose {}", self.thunderbird_exe_path, compose_arg);
            return Ok(());
        }

        let mut child = Command::new(&self.thunderbird_exe_path)
            .args(&["-compose", &compose_arg])
            .spawn()
            .map_err(|e| {
                AppError::new(ErrorKind::InternalServerError)
                    .with_message("Thunderbirdの起動に失敗しました。")
                    .with_action("Thunderbirdのパスが正しいことを確認してください。")
                    .with_source(e)
            })?;

        child.wait().map_err(|e| {
            AppError::new(ErrorKind::InternalServerError)
                .with_message("Thunderbirdプロセスの待機に失敗しました。")
                .with_action("システムリソースを確認してください。")
                .with_source(e)
        })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{
        email_address::EmailAddress,
        mail_objects::{MailBody, Subject},
    };

    #[test]
    fn test_compose_arg_building() {
        let adapter = ThunderbirdMailClientAdapter::new("thunderbird");
        
        let to = vec![EmailAddress::parse("test1@example.com").unwrap()];
        let cc = vec![EmailAddress::parse("test2@example.com").unwrap()];
        let subject = Subject::new("テスト件名").unwrap();
        let body = MailBody::new("テスト本文\n改行あり");
        
        let draft = MailDraft::new(to, cc, subject, body);
        let compose_arg = adapter.build_compose_arg(&draft);
        
        assert!(compose_arg.contains("to='test1@example.com'"));
        assert!(compose_arg.contains("cc='test2@example.com'"));
        assert!(compose_arg.contains("subject='テスト件名'"));
        assert!(compose_arg.contains("テスト本文\r\n改行あり"));
    }

    #[test]
    fn test_dry_run() {
        let adapter = ThunderbirdMailClientAdapter::new("thunderbird");
        
        let to = vec![EmailAddress::parse("test@example.com").unwrap()];
        let cc = vec![];
        let subject = Subject::new("テスト").unwrap();
        let body = MailBody::new("テスト本文");
        
        let draft = MailDraft::new(to, cc, subject, body);
        
        // ドライランは常に成功するはず
        adapter.compose_mail(&draft, true).unwrap();
    }
}