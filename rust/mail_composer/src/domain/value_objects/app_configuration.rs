use serde::{Deserialize, Serialize};
use share::error::{
    app_error::{AppError, AppResult},
    kind::ErrorKind,
};
use std::path::{Path, PathBuf};

/// アプリケーション設定を表現する値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppConfiguration {
    /// 差出人名
    pub from: String,
    /// 差出部署
    pub department: String,
    /// Thunderbird実行ファイルのパス
    pub thunderbird_exe: String,
    /// ログディレクトリ
    pub log_dir: String,
    /// 入力ディレクトリ
    pub input_dir: String,
    /// アドレスブックファイル名
    pub address_book_file: String,
    /// 出力ディレクトリ
    pub output_dir: String,
    /// 作業開始時間ファイル名
    pub start_time_file: String,
}

impl AppConfiguration {
    /// 設定値を検証する
    ///
    /// ## Returns
    /// * 成功時 - `Ok(())`
    /// * 失敗時 - 検証エラーのAppError
    pub fn validate(&self) -> AppResult<()> {
        if self.from.trim().is_empty() {
            return Err(AppError::new(ErrorKind::UnavailableForLegalReasons)
                .with_message("差出人名が設定されていません。")
                .with_action("config.jsonのfromフィールドに差出人名を設定してください。"));
        }

        if self.department.trim().is_empty() {
            return Err(AppError::new(ErrorKind::UnavailableForLegalReasons)
                .with_message("差出部署が設定されていません。")
                .with_action("config.jsonのdepartmentフィールドに部署名を設定してください。"));
        }

        if self.thunderbird_exe.trim().is_empty() {
            return Err(AppError::new(ErrorKind::UnavailableForLegalReasons)
                .with_message("Thunderbird実行ファイルのパスが設定されていません。")
                .with_action("config.jsonのthunderbird_exeフィールドにThunderbirdのパスを設定してください。"));
        }

        Ok(())
    }

    /// アドレスブックファイルのフルパスを取得する
    ///
    /// ## Returns
    /// * アドレスブックファイルの相対パス
    pub fn address_book_path(&self) -> PathBuf {
        Path::new(&self.input_dir).join(&self.address_book_file)
    }

    /// 作業開始時間ファイルのフルパスを取得する
    ///
    /// ## Returns
    /// * 作業開始時間ファイルの相対パス
    pub fn start_time_file_path(&self) -> PathBuf {
        Path::new(&self.input_dir).join(&self.start_time_file)
    }

    /// 出力ディレクトリのパスを取得する
    ///
    /// ## Returns
    /// * 出力ディレクトリのパス
    pub fn output_dir_path(&self) -> &Path {
        Path::new(&self.output_dir)
    }

    /// ログディレクトリのパスを取得する
    ///
    /// ## Returns
    /// * ログディレクトリのパス
    pub fn log_dir_path(&self) -> &Path {
        Path::new(&self.log_dir)
    }
}
