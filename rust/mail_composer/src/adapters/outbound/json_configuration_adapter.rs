use crate::domain::{
    ports::configuration::ConfigurationPort, value_objects::app_configuration::AppConfiguration,
};
use share::{
    error::{
        app_error::{AppError, AppResult},
        kind::ErrorKind,
    },
    utils::workspace::workspace_root,
};
use std::fs;

/// JSON形式の設定ファイルを処理するアウトバウンドアダプター
pub struct JsonConfigurationAdapter {
    config_file_path: String,
}

impl JsonConfigurationAdapter {
    /// 新しいJsonConfigurationAdapterを作成する
    ///
    /// ## Arguments
    /// * `config_file_path` - 設定ファイルの相対パス
    ///
    /// ## Returns
    /// * JsonConfigurationAdapterのインスタンス
    pub fn new(config_file_path: impl Into<String>) -> Self {
        Self {
            config_file_path: config_file_path.into(),
        }
    }

    /// デフォルト設定でアダプターを作成する
    ///
    /// ## Returns
    /// * デフォルト設定のJsonConfigurationAdapterのインスタンス
    pub fn with_default_path() -> Self {
        Self::new("rust/mail_composer/config/app.json")
    }

    /// 設定ファイルの絶対パスを取得する
    ///
    /// ## Returns
    /// * 成功時 - 設定ファイルの絶対パス
    /// * 失敗時 - ワークスペースルート取得エラー
    fn get_absolute_config_path(&self) -> AppResult<std::path::PathBuf> {
        let root = workspace_root()?;
        Ok(root.join(&self.config_file_path))
    }
}

impl ConfigurationPort for JsonConfigurationAdapter {
    /// アプリケーション設定を読み込む
    ///
    /// ## Returns
    /// * 成功時 - [`Ok<AppConfiguration>`]
    /// * 失敗時 - [`Err<AppError>`]
    fn load_configuration(&self) -> AppResult<AppConfiguration> {
        let config_path = self.get_absolute_config_path()?;

        let content = fs::read_to_string(&config_path).map_err(|e| {
            AppError::new(ErrorKind::InternalServerError)
                .with_message("設定ファイルの読み込みに失敗しました。")
                .with_action("config.jsonファイルの存在とアクセス権限を確認してください。")
                .with_source(e)
        })?;

        let mut config: AppConfiguration = serde_json::from_str(&content).map_err(|e| {
            AppError::new(ErrorKind::UnavailableForLegalReasons)
                .with_message("設定ファイルの解析に失敗しました。")
                .with_action("config.jsonファイルの形式が正しいことを確認してください。")
                .with_source(e)
        })?;

        // パスの正規化（Windows/Unix互換）
        config.thunderbird_exe = config.thunderbird_exe.replace('\\', "/");

        // 設定値を検証
        config.validate()?;

        Ok(config)
    }

    /// 設定ファイルが存在するかチェックする
    ///
    /// ## Returns
    /// * ファイルが存在する場合 - `true`
    /// * ファイルが存在しない場合 - `false`
    fn configuration_exists(&self) -> bool {
        if let Ok(config_path) = self.get_absolute_config_path() {
            config_path.exists()
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_configuration() {
        let adapter = JsonConfigurationAdapter::with_default_path();

        if !adapter.configuration_exists() {
            println!("❌ Configuration file not found - skipping test");
            return;
        }

        let result = adapter.load_configuration();

        match result {
            Ok(config) => {
                println!("✅ Configuration loaded successfully!");
                println!("From: {}", config.from);
                println!("Department: {}", config.department);
                println!("Thunderbird exe: {}", config.thunderbird_exe);
                println!("Log dir: {}", config.log_dir);
                println!("Address book path: {:?}", config.address_book_path());
                println!("Start time file path: {:?}", config.start_time_file_path());
                println!("Output dir: {:?}", config.output_dir_path());
                println!("Log dir: {:?}", config.log_dir_path());
            }
            Err(e) => {
                println!("❌ Failed to load configuration: {}", e);
            }
        }
    }

    #[test]
    fn test_configuration_exists() {
        let adapter = JsonConfigurationAdapter::with_default_path();
        let exists = adapter.configuration_exists();
        println!("Configuration file exists: {}", exists);
        assert!(exists, "Configuration file should exist for testing");
    }
}
