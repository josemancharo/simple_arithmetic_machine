use std::{error::Error, fmt::Display};

pub type SamError = Box<dyn Error>;

#[derive(Debug, Clone)]
pub struct ErrorWithMessage {
    pub message: String,
}

impl ErrorWithMessage {
    pub fn new(message: String) -> ErrorWithMessage {
        ErrorWithMessage {
            message,
        }
    }

    pub fn new_box(message: &str) -> Box<ErrorWithMessage> {
        Box::new(ErrorWithMessage::new(message.to_owned()))
    }
}

impl Display for ErrorWithMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for ErrorWithMessage {
    fn description(&self) -> &str {
        &self.message
    }
}