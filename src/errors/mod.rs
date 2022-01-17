use std::io;

use rustyline::error::ReadlineError;

#[derive(Debug)]
pub enum SamError {
    SyntaxError,
    TypeError,
    ReadlineError,
    IOError
}

impl From<ReadlineError> for SamError {
    fn from(_: ReadlineError) -> Self {
        SamError::ReadlineError
    }
}

impl From<io::Error> for SamError {
    fn from(_: io::Error) -> Self {
        SamError::IOError
    }
}