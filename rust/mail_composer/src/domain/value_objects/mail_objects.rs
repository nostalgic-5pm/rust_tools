use serde::{Deserialize, Serialize};
use share::error::{
    app_error::{AppError, AppResult},
    kind::ErrorKind,
};

/// メールの件名を表現する値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subject(String);

impl Subject {
    /// 件名を作成する
    ///
    /// ## Arguments
    /// * `subject` - 件名文字列
    ///
    /// ## Returns
    /// * 成功時 - `Ok<Subject>`
    /// * 失敗時 - `Err<AppError>`
    pub fn new(subject: impl Into<String>) -> AppResult<Self> {
        let subject = subject.into();
        if subject.trim().is_empty() {
            return Err(AppError::new(ErrorKind::UnavailableForLegalReasons)
                .with_message("件名が空です。")
                .with_action("適切な件名を設定してください。"));
        }
        Ok(Self(subject))
    }

    /// 件名文字列を取得する
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// メールの本文を表現する値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MailBody(String);

impl MailBody {
    /// 本文を作成する
    ///
    /// ## Arguments
    /// * `body` - 本文文字列
    ///
    /// ## Returns
    /// * MailBodyのインスタンス
    pub fn new(body: impl Into<String>) -> Self {
        Self(body.into())
    }

    /// 本文文字列を取得する
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Windows形式の改行コード（CRLF）に変換する
    pub fn to_crlf(&self) -> String {
        self.0.replace('\n', "\r\n")
    }
}

/// 時刻を表現する値オブジェクト（HH:MM形式）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkTime(String);

impl WorkTime {
    /// 時刻を作成する
    ///
    /// ## Arguments
    /// * `time` - 時刻文字列（HH:MM形式）
    ///
    /// ## Returns
    /// * 成功時 - `Ok<WorkTime>`
    /// * 失敗時 - `Err<AppError>`
    pub fn new(time: impl Into<String>) -> AppResult<Self> {
        let time = time.into();
        // 簡単なHH:MM形式の検証
        if !time.matches(':').count() == 1 || time.len() != 5 {
            return Err(AppError::new(ErrorKind::UnavailableForLegalReasons)
                .with_message("時刻の形式が不正です。")
                .with_action("HH:MM形式で時刻を指定してください。"));
        }
        Ok(Self(time))
    }

    /// 現在時刻を取得する
    pub fn now() -> AppResult<Self> {
        use chrono::Local;
        let now = Local::now().format("%H:%M").to_string();
        Self::new(now)
    }

    /// 時刻文字列を取得する
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 作業時間の範囲を表現する値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkTimeRange {
    start: WorkTime,
    end: WorkTime,
}

impl WorkTimeRange {
    /// 作業時間範囲を作成する
    ///
    /// ## Arguments
    /// * `start` - 開始時刻
    /// * `end` - 終了時刻
    ///
    /// ## Returns
    /// * WorkTimeRangeのインスタンス
    pub fn new(start: WorkTime, end: WorkTime) -> Self {
        Self { start, end }
    }

    /// 開始時刻を取得する
    pub fn start(&self) -> &WorkTime {
        &self.start
    }

    /// 終了時刻を取得する
    pub fn end(&self) -> &WorkTime {
        &self.end
    }

    /// 作業時間を文字列として表現する
    pub fn to_string(&self) -> String {
        format!("{}-{}", self.start.as_str(), self.end.as_str())
    }
}