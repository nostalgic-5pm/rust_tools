use crate::domain::interfaces::mail_config::MailConfigPort;
use crate::domain::value_objects::mail_config::MailConfig;
use share::{
    error::{
        app_error::{AppError, AppResult},
        kind::ErrorKind,
    },
    utils::workspace::workspace_root,
};
use std::collections::HashMap;
use std::fs;

pub struct JsonMailConfigAdapter {
    config_file_path: String,
}

impl JsonMailConfigAdapter {
    pub fn new() -> Self {
        Self {
            config_file_path: "rust/mail_composer/config/mail_templates.json".to_string(),
        }
    }
}

impl MailConfigPort for JsonMailConfigAdapter {
    fn load_mail_config(&self) -> AppResult<MailConfig> {
        let workspace_root = workspace_root().map_err(|e| {
            e.with_message("ワークスペースのルートディレクトリの取得に失敗しました。")
        })?;
        let path = workspace_root.join(&self.config_file_path);

        let content = fs::read_to_string(&path).map_err(|e| {
            AppError::new(ErrorKind::NotFound)
                .with_message("mail_config.jsonファイルの読み込みに失敗しました。")
                .with_action("ファイルの存在とアクセス権限を確認してください。")
                .with_source(e)
        })?;

        let raw_config: HashMap<String, serde_json::Value> = serde_json::from_str(&content)
            .map_err(|e| {
                AppError::new(ErrorKind::UnprocessableEntity)
                    .with_message("mail_config.jsonファイルの解析に失敗しました。")
                    .with_action("ファイルの形式が正しいことを確認してください。")
                    .with_source(e)
            })?;

        let mut mail_types = HashMap::new();
        for (key, value) in raw_config {
            let mail_type_config = serde_json::from_value(value).map_err(|e| {
                let message = format!("mail_configのmail type '{}'の解析に失敗しました。", key);
                AppError::new(ErrorKind::UnprocessableEntity)
                    .with_message(message)
                    .with_action("設定ファイルの形式を確認してください。")
                    .with_source(e)
            })?;
            mail_types.insert(key, mail_type_config);
        }

        Ok(MailConfig { mail_types })
    }
}
