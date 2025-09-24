use crate::error::kind::ErrorKind;
use derive_more::Display;
use serde::Serialize;
use std::borrow::Cow;
use thiserror::Error;

/// 本プロジェクト内で使用する結果型
///
/// [`Result<T, AppError>`]の型エイリアス
///
/// ## Examples
/// ```rust
/// use share::error::{app_error::{AppError, AppResult}, kind::ErrorKind};
///
/// fn some_operation() -> AppResult<String> {
///     Ok("success".to_string())
/// }
///
/// fn failing_operation() -> AppResult<i32> {
///     Err(AppError::new(ErrorKind::BadRequest))
/// }
/// ```
pub type AppResult<T> = Result<T, AppError>;

/// 本プロジェクト内で使用するエラー構造体
///
/// ## Fields
/// * `kind` - エラー種別（[`ErrorKind`]）
/// * `message` - ユーザー向けのエラーメッセージ
/// * `action` - ユーザー向けの対処法（オプション）
/// * `source` - 元となったエラー（オプション、シリアライズ対象外）
///
/// ## Examples
/// ```rust
/// use share::error::{app_error::AppError, kind::ErrorKind};
///
/// let error = AppError::new(ErrorKind::BadRequest)
///     .with_message("無効なリクエストです。")
///     .with_action("入力内容を確認してください。");
/// ```
#[derive(Debug, Error, Serialize, Display)]
#[display("kind: {}, message: {message}", kind.as_str())]
pub struct AppError {
    pub kind: ErrorKind,
    pub message: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Cow<'static, str>>,
    #[serde(skip_serializing)]
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl AppError {
    /// 新しい[`AppError`]を作成する
    ///
    /// デフォルトのエラーメッセージ「エラーが発生しました。」で初期化する
    ///
    /// ## Arguments
    /// * `kind` - エラー種別（[`ErrorKind`]）
    ///
    /// ## Returns
    /// * 新しい[`AppError`]インスタンス
    ///
    /// ## Examples
    /// ```rust
    /// use share::error::{app_error::AppError, kind::ErrorKind};
    ///
    /// let error = AppError::new(ErrorKind::NotFound);
    /// assert_eq!(error.kind, ErrorKind::NotFound);
    /// ```
    #[inline]
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            message: Cow::Borrowed("エラーが発生しました。"),
            action: None,
            source: None,
        }
    }

    /// エラーメッセージを設定する
    ///
    /// ## Arguments
    /// * `msg` - 設定するエラーメッセージ
    ///
    /// ## Returns
    /// * メッセージが設定された[`AppError`]インスタンス
    ///
    /// ## Notes
    /// * このメソッドは、[`AppError`]インスタンス生成後にチェーンして呼び出す
    ///
    /// ## Examples
    /// ```rust
    /// use share::error::{app_error::AppError, kind::ErrorKind};
    ///
    /// let error = AppError::new(ErrorKind::NotFound)
    ///     .with_message("指定されたリソースが見つかりません。");
    /// assert_eq!(error.message, "指定されたリソースが見つかりません。");
    /// ```
    #[inline]
    pub fn with_message<S>(mut self, msg: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.message = msg.into();
        self
    }

    /// ユーザー向けの対処法を設定する
    ///
    /// ## Arguments
    /// * `action` - 設定する対処法
    ///
    /// ## Returns
    /// * 対処法が設定された[`AppError`]インスタンス
    ///
    /// ## Notes
    /// * このメソッドは、[`AppError`]インスタンス生成後にチェーンして呼び出す
    ///
    /// ## Examples
    /// ```rust
    /// use share::error::{app_error::AppError, kind::ErrorKind};
    ///
    /// let error = AppError::new(ErrorKind::Forbidden)
    ///     .with_message("アクセス権限がありません。")
    ///     .with_action("管理者に連絡してください。");
    /// assert!(error.action.is_some());
    /// ```
    pub fn with_action<S>(mut self, action: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        self.action = Some(action.into());
        self
    }

    /// 元のエラーを設定する
    ///
    /// 任意のエラー値を引数で渡す
    /// 設定されたエラーは`Box`に格納される
    ///
    /// ## Arguments
    /// * `source` - 設定する元エラー
    ///
    /// ## Returns
    /// * 元のエラーが設定された[`AppError`]インスタンス
    ///
    /// ## Notes
    /// * このメソッドは[`AppError`]インスタンス生成後にチェーンして呼び出す
    ///
    /// ## Examples
    /// ```rust
    /// use share::error::{app_error::AppError, kind::ErrorKind};
    /// use std::io;
    ///
    /// let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    /// let error = AppError::new(ErrorKind::InternalServerError)
    ///     .with_message("ファイル処理エラーが発生しました。")
    ///     .with_source(io_error);
    /// assert!(error.source.is_some());
    /// ```
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Into<Box<dyn std::error::Error + Send + Sync>>,
    {
        self.source = Some(source.into());
        self
    }
}
