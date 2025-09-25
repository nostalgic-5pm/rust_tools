use crate::domain::value_objects::{
    email_address::EmailAddress,
    mail_objects::{MailBody, Subject},
};

/// メールドラフトを表現するエンティティ
#[derive(Debug, Clone)]
pub struct MailDraft {
    to: Vec<EmailAddress>,
    cc: Vec<EmailAddress>,
    subject: Subject,
    body: MailBody,
}

impl MailDraft {
    /// 新しいメールドラフトを作成する
    ///
    /// ## Arguments
    /// * `to` - TO宛先のリスト
    /// * `cc` - CC宛先のリスト
    /// * `subject` - 件名
    /// * `body` - 本文
    ///
    /// ## Returns
    /// * MailDraftのインスタンス
    pub fn new(
        to: Vec<EmailAddress>,
        cc: Vec<EmailAddress>,
        subject: Subject,
        body: MailBody,
    ) -> Self {
        Self { to, cc, subject, body }
    }

    /// TO宛先を取得する
    pub fn to(&self) -> &[EmailAddress] {
        &self.to
    }

    /// CC宛先を取得する
    pub fn cc(&self) -> &[EmailAddress] {
        &self.cc
    }

    /// 件名を取得する
    pub fn subject(&self) -> &Subject {
        &self.subject
    }

    /// 本文を取得する
    pub fn body(&self) -> &MailBody {
        &self.body
    }

    /// TO宛先をカンマ区切りの文字列として取得する
    pub fn to_addresses_as_string(&self) -> String {
        self.to
            .iter()
            .map(|addr| addr.as_str())
            .collect::<Vec<_>>()
            .join(",")
    }

    /// CC宛先をカンマ区切りの文字列として取得する
    pub fn cc_addresses_as_string(&self) -> String {
        self.cc
            .iter()
            .map(|addr| addr.as_str())
            .collect::<Vec<_>>()
            .join(",")
    }
}