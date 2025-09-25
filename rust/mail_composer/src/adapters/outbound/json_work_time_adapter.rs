use crate::domain::{
    entities::start_time_map::StartTimeMap, ports::work_time::WorkTimePort,
    value_objects::mail_objects::WorkTime,
};
use chrono::NaiveDate;
use share::{
    error::{
        app_error::{AppError, AppResult},
        kind::ErrorKind,
    },
    utils::workspace::{ensure_directory_exists, workspace_path},
};
use std::{fs, path::PathBuf};

/// JSON形式で作業時間を管理するアウトバウンドアダプター
pub struct JsonWorkTimeAdapter {
    log_dir: String,
    file_name: String,
}

impl JsonWorkTimeAdapter {
    /// 新しいJsonWorkTimeAdapterを作成する
    ///
    /// ## Arguments
    /// * `log_dir` - ログディレクトリのパス
    /// * `file_name` - ファイル名
    ///
    /// ## Returns
    /// * JsonWorkTimeAdapterのインスタンス
    pub fn new(log_dir: impl Into<String>, file_name: impl Into<String>) -> Self {
        Self {
            log_dir: log_dir.into(),
            file_name: file_name.into(),
        }
    }

    /// デフォルト設定でアダプターを作成する
    ///
    /// ## Returns
    /// * デフォルト設定のJsonWorkTimeAdapterのインスタンス
    pub fn with_default_settings() -> Self {
        Self::new("rust/mail_composer/data", "work_times.json")
    }

    /// ログファイルのパスを取得する
    fn get_output_file_path(&self) -> AppResult<PathBuf> {
        let dir_path = workspace_path(&self.log_dir)?;
        ensure_directory_exists(&dir_path)?;
        Ok(dir_path.join(&self.file_name))
    }

    /// StartTimeMapを読み込む
    fn load_start_time_map(&self) -> AppResult<StartTimeMap> {
        let path = self.get_output_file_path()?;
        if !path.exists() {
            return Ok(StartTimeMap::new());
        }

        let content = fs::read_to_string(&path).map_err(|e| {
            AppError::new(ErrorKind::InternalServerError)
                .with_message("作業時間ファイルの読み込みに失敗しました。")
                .with_action("ファイルの存在とアクセス権限を確認してください。")
                .with_source(e)
        })?;

        let map: StartTimeMap = serde_json::from_str(&content).map_err(|e| {
            AppError::new(ErrorKind::UnavailableForLegalReasons)
                .with_message("作業時間ファイルの解析に失敗しました。")
                .with_action("ファイルの形式が正しいことを確認してください。")
                .with_source(e)
        })?;

        Ok(map)
    }

    /// StartTimeMapを保存する
    fn save_start_time_map(&self, map: &StartTimeMap) -> AppResult<()> {
        let path = self.get_output_file_path()?;

        let json = serde_json::to_string_pretty(map).map_err(|e| {
            AppError::new(ErrorKind::InternalServerError)
                .with_message("JSONへの変換に失敗しました。")
                .with_action("データの内容を確認してください。")
                .with_source(e)
        })?;

        fs::write(path, json).map_err(|e| {
            AppError::new(ErrorKind::InternalServerError)
                .with_message("作業時間ファイルの書き込みに失敗しました。")
                .with_action("ディスクの容量とアクセス権限を確認してください。")
                .with_source(e)
        })?;

        Ok(())
    }
}

impl WorkTimePort for JsonWorkTimeAdapter {
    fn save_start_time(&self, date: NaiveDate, start_time: &WorkTime) -> AppResult<()> {
        let mut map = self.load_start_time_map()?;
        map.set_start_time(date.to_string(), start_time.as_str().to_string());
        self.save_start_time_map(&map)
    }

    fn load_start_time(&self, date: NaiveDate) -> AppResult<Option<WorkTime>> {
        let map = self.load_start_time_map()?;
        if let Some(time_str) = map.get_start_time(&date.to_string()) {
            let work_time = WorkTime::new(time_str)?;
            Ok(Some(work_time))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work_time_roundtrip() {
        let adapter = JsonWorkTimeAdapter::with_default_settings();
        let work_time = WorkTime::new("09:30").unwrap();

        // 今日の時間を保存
        adapter.save_today_start_time(&work_time).unwrap();

        // 今日の時間を読み込み
        let loaded_time = adapter.load_today_start_time().unwrap();

        assert!(loaded_time.is_some());
        assert_eq!(loaded_time.unwrap().as_str(), "09:30");
    }
}
