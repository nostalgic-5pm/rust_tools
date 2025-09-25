use share::error::app_error::AppResult;
use crate::domain::value_objects::mail_objects::WorkTime;
use chrono::NaiveDate;

/// 作業時間管理のためのポート（セカンダリポート）
pub trait WorkTimePort {
    /// 指定日の作業開始時刻を保存する
    ///
    /// ## Arguments
    /// * `date` - 対象日付
    /// * `start_time` - 開始時刻
    ///
    /// ## Returns
    /// * 成功時 - `Ok(())`
    /// * 失敗時 - `Err<AppError>`
    fn save_start_time(&self, date: NaiveDate, start_time: &WorkTime) -> AppResult<()>;

    /// 今日の作業開始時刻を保存する
    ///
    /// ## Arguments
    /// * `start_time` - 開始時刻
    ///
    /// ## Returns
    /// * 成功時 - `Ok(())`
    /// * 失敗時 - `Err<AppError>`
    fn save_today_start_time(&self, start_time: &WorkTime) -> AppResult<()> {
        use chrono::Local;
        let today = Local::now().date_naive();
        self.save_start_time(today, start_time)
    }

    /// 指定日の作業開始時刻を読み込む
    ///
    /// ## Arguments
    /// * `date` - 対象日付
    ///
    /// ## Returns
    /// * 成功時 - `Ok<Option<WorkTime>>` (記録がない場合はNone)
    /// * 失敗時 - `Err<AppError>`
    fn load_start_time(&self, date: NaiveDate) -> AppResult<Option<WorkTime>>;

    /// 今日の作業開始時刻を読み込む
    ///
    /// ## Returns
    /// * 成功時 - `Ok<Option<WorkTime>>` (記録がない場合はNone)
    /// * 失敗時 - `Err<AppError>`
    fn load_today_start_time(&self) -> AppResult<Option<WorkTime>> {
        use chrono::Local;
        let today = Local::now().date_naive();
        self.load_start_time(today)
    }
}