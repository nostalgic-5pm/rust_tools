use serde::{Deserialize, Serialize};
use share::error::{
    app_error::{AppError, AppResult},
    kind::ErrorKind,
};

/// メールアドレスを表現する値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// EmailAddressを表現する文字列から[`EmailAddress`]構造体を生成する
    ///
    /// ## Arguments
    /// * `email_address` - 生成対象のメールアドレスを表現する文字列
    ///
    /// ## Returns
    /// * 成功時 - [`Ok<EmailAddress>`]
    /// * 失敗時 - [`Err<AppError>`]
    pub fn parse(email_address: impl Into<String>) -> AppResult<Self> {
        let email_address = email_address.into();
        // TODO: より厳密なバリデーションを実装する
        if !email_address.contains('@') {
            return Err(AppError::new(ErrorKind::UnavailableForLegalReasons)
                .with_message(format!(
                    "メールアドレスの形式が不正です。詳細: {email_address}"
                ))
                .with_action("正しいメールアドレスを指定してください。"));
        }
        Ok(Self(email_address))
    }

    /// [`EmailAddress`]を表現する文字列を返す
    ///
    /// ## Arguments
    /// * `&self` - 文字列を取得対象の[`EmailAddress`]
    ///
    /// ## Returns
    /// * 文字列を取得対象の[`EmailAddress`]を表現する文字列の参照
    ///
    /// ## Examples
    /// ```rust
    /// use mail_composer::domain::value_objects::email_address::EmailAddress;
    /// let email = EmailAddress::parse("sample@example.com").unwrap();
    /// assert_eq!(email.as_str(), "sample@example.com");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
