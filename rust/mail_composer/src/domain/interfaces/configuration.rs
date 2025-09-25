use share::error::app_error::AppResult;
use crate::domain::value_objects::app_configuration::AppConfiguration;

/// 設定読み込みのためのポート（セカンダリポート）
pub trait ConfigurationPort {
    /// アプリケーション設定を読み込む
    ///
    /// ## Returns
    /// * 成功時 - [`Ok<AppConfiguration>`]
    /// * 失敗時 - [`Err<AppError>`]
    fn load_configuration(&self) -> AppResult<AppConfiguration>;

    /// 設定ファイルが存在するかチェックする
    ///
    /// ## Returns
    /// * ファイルが存在する場合 - `true`
    /// * ファイルが存在しない場合 - `false`
    fn configuration_exists(&self) -> bool;
}
