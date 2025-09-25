use share::error::app_error::AppResult;
use crate::domain::entities::mail_draft::MailDraft;

/// メール送信のためのポート（セカンダリポート）
pub trait MailClientPort {
    /// メールドラフトを作成・送信する
    ///
    /// ## Arguments
    /// * `draft` - メールドラフト
    /// * `is_dry_run` - ドライランモード（true の場合、実際の送信は行わない）
    ///
    /// ## Returns
    /// * 成功時 - `Ok(())`
    /// * 失敗時 - `Err<AppError>`
    fn compose_mail(&self, draft: &MailDraft, is_dry_run: bool) -> AppResult<()>;
}