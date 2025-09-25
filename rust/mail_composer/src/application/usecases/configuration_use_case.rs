use crate::domain::{
    interfaces::configuration::ConfigurationPort,
    value_objects::app_configuration::AppConfiguration,
};
use share::error::app_error::AppResult;

/// 設定管理のユースケース
pub struct ConfigurationUseCase<C: ConfigurationPort> {
    configuration_port: C,
}

impl<C: ConfigurationPort> ConfigurationUseCase<C> {
    /// 新しいConfigurationUseCaseを作成する
    ///
    /// ## Arguments
    /// * `configuration_port` - 設定読み込み用のポート
    ///
    /// ## Returns
    /// * ConfigurationUseCaseのインスタンス
    pub fn new(configuration_port: C) -> Self {
        Self { configuration_port }
    }

    /// アプリケーション設定を取得する
    ///
    /// ## Returns
    /// * 成功時 - `Ok<AppConfiguration>`
    /// * 失敗時 - `Err<AppError>`
    pub fn get_configuration(&self) -> AppResult<AppConfiguration> {
        self.configuration_port.load_configuration()
    }

    /// 設定ファイルが利用可能かチェックする
    ///
    /// ## Returns
    /// * 設定ファイルが存在する場合 - `true`
    /// * 設定ファイルが存在しない場合 - `false`
    pub fn is_configuration_available(&self) -> bool {
        self.configuration_port.configuration_exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::outbound::json_configuration_adapter::JsonConfigurationAdapter;

    #[test]
    fn test_configuration_use_case() {
        let adapter = JsonConfigurationAdapter::with_default_path();
        let use_case = ConfigurationUseCase::new(adapter);

        // 設定ファイルの存在確認
        let is_available = use_case.is_configuration_available();
        println!("Configuration is available: {}", is_available);

        if is_available {
            // 設定の読み込みテスト
            match use_case.get_configuration() {
                Ok(config) => {
                    println!("✅ Configuration use case test passed!");
                    println!("Loaded configuration:");
                    println!("  - From: {}", config.from);
                    println!("  - Department: {}", config.department);
                    println!("  - Thunderbird: {}", config.thunderbird_exe);
                    println!("  - Address book: {:?}", config.address_book_path());
                }
                Err(e) => {
                    println!("❌ Configuration use case test failed: {}", e);
                    panic!("Configuration should be loadable");
                }
            }
        } else {
            println!("⚠️  Configuration file not available - skipping detailed test");
        }
    }
}
