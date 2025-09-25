use crate::domain::value_objects::email_address::EmailAddress;
use share::error::app_error::AppResult;

/// アドレスブック操作のためのポート（セカンダリポート）
pub trait AddressBookPort {
    /// AddressBookからメールアドレスを取得する
    ///
    /// ## Arguments
    /// * `key_name` - 取得対象のメールアドレスに対応する名前(AddressBookのキー)
    ///
    /// ## Returns
    /// * 成功時 - [`Ok<EmailAddress>`]
    /// * 失敗時 - [`Err<AppError>`]
    fn resolve(&self, key_name: &str) -> AppResult<EmailAddress>;

    /// AddressBookから複数のメールアドレスを取得する
    ///
    /// ## Arguments
    /// * `key_names` - 取得対象のメールアドレスに対応する名前(AddressBookのキー)のスライス
    ///
    /// ## Returns
    /// * 成功時 - [`Ok<Vec<EmailAddress>>`]
    /// * 失敗時 - [`Err<AppError>`]
    fn resolve_many(&self, key_names: &[&str]) -> AppResult<Vec<EmailAddress>> {
        key_names
            .iter()
            .map(|key_name| self.resolve(key_name))
            .collect()
    }
}
