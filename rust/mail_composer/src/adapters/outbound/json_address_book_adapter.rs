use crate::domain::{
    ports::address_book::AddressBookPort, value_objects::email_address::EmailAddress,
};
use serde::{Deserialize, Serialize};
use share::{
    error::{
        app_error::{AppError, AppResult},
        kind::ErrorKind,
    },
    utils::workspace::workspace_root,
};
use std::{collections::BTreeMap, fs, path::Path};

/// AddressBookエントリを表現する構造体
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddressBookEntry {
    pub name: String,
    pub address: String,
}

/// JSON形式のアドレスブックを処理するアウトバウンドアダプター
pub struct JsonAddressBookAdapter {
    map: BTreeMap<String, String>,
    entries: Vec<AddressBookEntry>,
}

impl JsonAddressBookAdapter {
    /// 指定されたパスからAddressBookを読み込む
    ///
    /// ## Arguments
    /// * `address_book` - AddressBookのパスを表現する`Path`
    ///
    /// ## Returns
    /// * 成功時 - `Ok<JsonAddressBookAdapter>`
    /// * 失敗時 - `Err<AppError>`
    pub fn load_from_address_book(address_book: &Path) -> AppResult<Self> {
        let root = workspace_root()?;
        let path = root.join(address_book);
        let content = fs::read_to_string(&path).map_err(|e| {
            AppError::new(ErrorKind::InternalServerError)
                .with_message("AddressBookファイルの読み込みに失敗しました。")
                .with_action("ファイルパスの存在とアクセス権限を確認してください。")
                .with_source(e)
        })?;

        let entries: Vec<AddressBookEntry> = serde_json::from_str(&content).map_err(|e| {
            AppError::new(ErrorKind::UnavailableForLegalReasons)
                .with_message("AddressBookの解析に失敗しました。")
                .with_action("JSONファイルの形式が正しいことを確認してください。期待される形式: [{\"name\": \"...\", \"address\": \"...\"}]")
                .with_source(e)
        })?;

        // 重複チェック
        let mut names = std::collections::HashSet::new();
        for entry in &entries {
            if !names.insert(&entry.name) {
                return Err(AppError::new(ErrorKind::UnavailableForLegalReasons)
                    .with_message("重複する名前が見つかりました。")
                    .with_action("AddressBook内の名前は一意である必要があります。"));
            }
        }

        // Vec<AddressBookEntry>をBTreeMap<String, String>に変換
        let map = entries
            .iter()
            .map(|entry| (entry.name.clone(), entry.address.clone()))
            .collect();

        Ok(Self { map, entries })
    }

    /// 全てのエントリを取得する
    ///
    /// ## Returns
    /// * 全てのAddressBookエントリのスライス
    pub fn entries(&self) -> &[AddressBookEntry] {
        &self.entries
    }

    /// 名前の一覧を取得する
    ///
    /// ## Returns
    /// * 登録されている名前の一覧
    pub fn names(&self) -> Vec<&str> {
        self.map.keys().map(|s| s.as_str()).collect()
    }

    /// AddressBookの内容を表示する（デバッグ用）
    ///
    /// ## Returns
    /// * 成功時 - `Ok(())`
    /// * 失敗時 - `Err<AppError>`
    pub fn display_contents(&self) -> AppResult<()> {
        println!("=== AddressBook Contents ===");
        for entry in &self.entries {
            println!("Name: {}, Address: {}", entry.name, entry.address);
        }
        println!("Total entries: {}", self.entries.len());
        Ok(())
    }
}

impl AddressBookPort for JsonAddressBookAdapter {
    /// AddressBookからメールアドレスを取得する
    ///
    /// ## Arguments
    /// * `key_name` - 取得対象のメールアドレスに対応する名前(AddressBookのキー)
    ///
    /// ## Returns
    /// * 成功時 - `Ok<EmailAddress>`
    /// * 失敗時 - `Err<AppError>`
    fn resolve(&self, key_name: &str) -> AppResult<EmailAddress> {
        let address = self.map.get(key_name).ok_or_else(|| {
            AppError::new(ErrorKind::NotFound)
                .with_message("指定された名前に対応するメールアドレスが見つかりません。")
                .with_action("AddressBookの内容と指定した名前を確認してください。")
        })?;
        // 文字列のクローンを避けて、参照から直接EmailAddressを作成
        EmailAddress::parse(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_load_address_book() {
        let path = Path::new("rust/mail_composer/config/address_book.json");
        let result = JsonAddressBookAdapter::load_from_address_book(path);

        match result {
            Ok(address_book) => {
                println!("✅ AddressBook loaded successfully!");
                let _ = address_book.display_contents();

                // テスト: "○○さん"を検索
                match address_book.resolve("○○さん") {
                    Ok(email) => println!("✅ Found email for '○○さん': {}", email.as_str()),
                    Err(e) => println!("❌ Error resolving '○○さん': {}", e),
                }

                // テスト: 存在しない名前を検索
                match address_book.resolve("存在しない人") {
                    Ok(_) => println!("❌ Should not find non-existent person"),
                    Err(e) => println!("✅ Expected error for non-existent person: {}", e),
                }
            }
            Err(e) => {
                println!("❌ Failed to load AddressBook: {}", e);
            }
        }
    }
}
