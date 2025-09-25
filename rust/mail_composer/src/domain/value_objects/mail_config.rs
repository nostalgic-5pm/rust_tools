use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct MailConfig {
    pub mail_types: HashMap<String, MailTypeConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MailTypeConfig {
    pub to_names: Vec<String>,
    pub cc_names: Vec<String>,
    pub subject_template: String,
    pub body_template: String,
}

impl MailConfig {
    pub fn get_mail_type(&self, mail_type: &str) -> Option<&MailTypeConfig> {
        self.mail_types.get(mail_type)
    }
}

impl MailTypeConfig {
    pub fn format_subject(&self, department: &str, from: &str, time: &str) -> String {
        self.subject_template
            .replace("{department}", department)
            .replace("{from}", from)
            .replace("{time}", time)
    }

    pub fn format_body(&self, work_time: Option<&str>) -> String {
        match work_time {
            Some(time) => self.body_template.replace("{work_time}", time),
            None => self.body_template.to_string(),
        }
    }
}