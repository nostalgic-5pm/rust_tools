use crate::domain::value_objects::mail_config::MailConfig;
use share::error::app_error::AppError;

pub trait MailConfigPort {
    fn load_mail_config(&self) -> Result<MailConfig, AppError>;
}