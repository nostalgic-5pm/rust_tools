use crate::error::{
    app_error::{AppError, AppResult},
    kind::ErrorKind,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

/// ワークスペースのルートディレクトリを返す
///
/// ## Arguments
/// * `()` - 引数なし
///
/// ## Returns
/// * 成功時 - ワークスペースのルートディレクトリのパスを表現する`PathBuf`
/// * 失敗時 - AppError
pub fn workspace_root() -> AppResult<PathBuf> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Ok(root) = find_workspace_root_from(&manifest_dir) {
        Ok(root)
    } else {
        Err(AppError::new(ErrorKind::NotFound)
            .with_message("ワークスペースのルートディレクトリが見つかりません。")
            .with_action(
                "プロジェクト最上階層のCargo.tomlファイルにワークスペース設定があることを確認してください。",
            ))
    }
}

/// 指定されたディレクトリからワークスペースのルートディレクトリまでを探索する
///
/// ## Arguments
/// * `start_dir` - 探索を開始するディレクトリを表現する`Path`の参照
///
/// ## Returns
/// * 成功時 - ワークスペースのルートディレクトリのパスを表現する`PathBuf`
/// * 失敗時 - AppError
pub fn find_workspace_root_from(start_dir: &Path) -> AppResult<PathBuf> {
    let mut dir = start_dir.to_path_buf();
    loop {
        let cargo_toml = dir.join("Cargo.toml");

        if cargo_toml.is_file() {
            match has_workspace_section(&cargo_toml) {
                Ok(true) => return Ok(dir),
                Ok(false) => {}
                Err(e) => return Err(e),
            }
        }
        if !dir.pop() {
            return Err(AppError::new(ErrorKind::NotFound)
                .with_message("ワークスペースのルートディレクトリが見つかりません。")
                .with_action(
                    "プロジェクト最上階層のCargo.tomlファイルにワークスペース設定があることを確認してください。",
                ));
        }
    }
}

/// `Cargo.toml`ファイル内に`[workspace]`セクションが含まれるか判定する
///
/// ## Arguments
/// * `cargo_toml` - 確認する`Cargo.toml`ファイルのパスを表現する`Path`
///
/// ## Returns
/// * 成功時 - [workspace]セクションの存在を示すbool値
/// * - `True` - `[workspace]`セクションが存在する
/// * - `False` - `[workspace]`セクションが存在しない
/// * 失敗時 - ファイル読み込みエラー時のAppError
fn has_workspace_section(cargo_toml: &Path) -> AppResult<bool> {
    let contents = fs::read_to_string(cargo_toml).map_err(|e| {
        AppError::new(ErrorKind::InternalServerError)
            .with_message("Cargo.tomlファイルの読み込みに失敗しました。")
            .with_action("Cargo.tomlファイルの存在およびアクセス権限を確認してください。")
            .with_source(e)
    })?;

    let has_workspace = contents
        .lines()
        .map(|line| line.trim())
        .any(|line| line == "[workspace]" || line.starts_with("[workspace."));

    Ok(has_workspace)
}

/// 指定されたパスにディレクトリが存在することを確認し、存在しない場合は作成する
///
/// ## Arguments
/// * `path` - 確認/作成するディレクトリのパス
///
/// ## Returns
/// * 成功時 - ()
/// * 失敗時 - ディレクトリ作成に失敗した場合またはパスがファイルの場合のAppError
pub fn ensure_directory_exists<P: AsRef<Path>>(path: P) -> AppResult<()> {
    let path = path.as_ref();

    if path.exists() {
        if !path.is_dir() {
            return Err(AppError::new(ErrorKind::InternalServerError)
                .with_message("パスが存在しますが、ディレクトリではありません。")
                .with_action("指定されたパスがファイルでないことを確認し、適切なディレクトリパスを指定してください。"));
        }
        return Ok(());
    }

    fs::create_dir_all(path).map_err(|e| {
        AppError::new(ErrorKind::InternalServerError)
            .with_message("ディレクトリの作成に失敗しました。")
            .with_action("ディレクトリの作成権限があることを確認し、親ディレクトリが存在することを確認してください。")
            .with_source(e)
    })
}

/// ワークスペースルートからの相対パスを絶対パスに変換する
///
/// ## Arguments
/// * `relative_path` - 変換対象の相対パス
///
/// ## Returns
/// * 成功時 - ワークスペースルートと結合された絶対パスの`PathBuf`
/// * 失敗時 - ワークスペースルートの取得に失敗した場合のAppError
pub fn workspace_path<P: AsRef<Path>>(relative_path: P) -> AppResult<PathBuf> {
    let root = workspace_root()?;
    Ok(root.join(&relative_path))
}

/// 絶対パスをワークスペースからの相対パスに変換する
///
/// ## Arguments
/// * `absolute_path` - 変換対象の絶対パス
///
/// ## Returns
/// * 成功時 - ワークスペースルートからの相対パスを表現する`PathBuf`
/// * 失敗時 - パスがワークスペース内に存在しない場合のAppError
pub fn relative_to_workspace<P: AsRef<Path>>(absolute_path: P) -> AppResult<PathBuf> {
    let workspace = workspace_root()?;
    let absolute = absolute_path.as_ref();

    absolute
        .strip_prefix(&workspace)
        .map(|p| p.to_path_buf())
        .map_err(|e| {
            AppError::new(ErrorKind::UnprocessableEntity)
                .with_message("パスがワークスペース内に存在しません。")
                .with_action("ワークスペース内の有効なパスを指定してください。")
                .with_source(e)
        })
}

#[cfg(test)]
mod ut {
    use super::*;

    #[test]
    fn find_manifest_dir() {
        let root = workspace_root().unwrap();
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        assert!(manifest.starts_with(&root));
    }
}
