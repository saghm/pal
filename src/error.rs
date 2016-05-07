use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    err: ErrorType,
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}: {}", self.err, self.message)
    }
}

impl Error {
    pub fn new(t: ErrorType, s: &str) -> Self {
        Error { err: t, message: String::from(s) }
    }

    pub fn type_error(s: &str) -> Self {
        Self::new(ErrorType::Type, s)
    }

    pub fn undef_var_error(s: &str) -> Self {
        Self::new(ErrorType::UndefVar, s)
    }
}

#[derive(Debug)]
pub enum ErrorType {
    Type,
    UndefVar,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorType::Type => write!(fmt, "TypeError"),
            ErrorType::UndefVar => write!(fmt, "UndefVarError"),
        }
    }
}
