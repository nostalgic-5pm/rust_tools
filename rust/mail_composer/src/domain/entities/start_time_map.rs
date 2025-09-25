use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// 作業開始時間を管理するエンティティ
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StartTimeMap(pub BTreeMap<String, String>);

impl StartTimeMap {
    /// 新しいStartTimeMapを作成する
    pub fn new() -> Self {
        Self::default()
    }

    /// 指定されたキーに対する開始時間を設定する
    pub fn set_start_time(&mut self, key: String, time: String) {
        self.0.insert(key, time);
    }

    /// 指定されたキーの開始時間を取得する
    pub fn get_start_time(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    /// 全ての開始時間エントリを取得する
    pub fn entries(&self) -> &BTreeMap<String, String> {
        &self.0
    }
}
