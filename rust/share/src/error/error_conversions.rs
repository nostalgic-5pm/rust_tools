use crate::error::{app_error::AppError, kind::ErrorKind};

impl From<anyhow::Error> for AppError {
    fn from(value: anyhow::Error) -> Self {
        AppError::new(ErrorKind::InternalServerError)
            .with_message("外部ライブラリでエラーが発生しました。")
            .with_source("システム管理者にお問い合わせください。")
            .with_source(value)
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        let (message, action) = match value.kind() {
            std::io::ErrorKind::NotFound => (
                "指定されたファイルまたはディレクトリが見つかりません。",
                "ファイルパスを確認してください。",
            ),
            std::io::ErrorKind::PermissionDenied => (
                "ファイルへのアクセス権限がありません。",
                "ファイルの権限設定を確認してください。",
            ),
            std::io::ErrorKind::AlreadyExists => (
                "ファイルまたはディレクトリが既に存在します。",
                "別の名前を使用するか、既存のファイルを削除してください。",
            ),
            std::io::ErrorKind::InvalidInput => (
                "無効な入力が指定されました。",
                "入力内容を確認してください。",
            ),
            _ => (
                "ファイル操作中にエラーが発生しました。",
                "ディスク容量やファイル権限を確認してください。",
            ),
        };

        AppError::new(ErrorKind::InternalServerError)
            .with_message(message)
            .with_action(action)
            .with_source(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        AppError::new(ErrorKind::UnprocessableEntity)
            .with_message("JSONの処理中にエラーが発生しました。")
            .with_action("JSONの形式を確認してください。")
            .with_source(value)
    }
}

impl From<calamine::XlsxError> for AppError {
    /// [`XlsxError`]を[`AppError`]に変換する
    ///
    /// ## Arguments
    /// * `value` - 変換対象の[`calamine::XlsxError`]
    ///
    /// ## Returns
    /// * 変換後の[`AppError`]
    fn from(value: calamine::XlsxError) -> Self {
        AppError::new(ErrorKind::InternalServerError)
            .with_message(format!("Excelファイルの読み込み中にエラーが発生しました。"))
            .with_action("Excelファイルの形式を確認してください。")
            .with_source(value)
    }
}
