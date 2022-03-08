use std::fmt;
use std::ops::Deref;

#[derive(Debug)]
pub struct Error {
    line_number: usize,
    msg: String,
}

pub struct ErrorLogger {
    errors: Vec<Error>,
}

impl ErrorLogger {
    pub fn new() -> Self {
        Self { errors: vec![] }
    }

    pub fn push(&mut self, line_number: usize, msg: &str) {
        self.errors.push(Error {
            line_number,
            msg: msg.to_owned(),
        })
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

impl Deref for ErrorLogger {
    type Target = Vec<Error>;

    fn deref(&self) -> &Self::Target {
        &self.errors
    }
}

impl fmt::Display for Error {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, line {}", self.msg, self.line_number)
    }
}
