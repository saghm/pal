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

    pub fn argument_error(s: &str) -> Self {
        Self::new(ErrorType::ArgumentError, s)
    }

    pub fn redef_func_error(s: &str) -> Self {
        Self::new(ErrorType::RedefFunc, s)
    }

    pub fn type_error(s: &str) -> Self {
        Self::new(ErrorType::Type, s)
    }

    pub fn undef_func_error(s: &str) -> Self {
        Self::new(ErrorType::UndefFunc, s)
    }

    pub fn undef_var_error(s: &str) -> Self {
        Self::new(ErrorType::UndefVar, s)
    }
}

#[derive(Debug)]
pub enum ErrorType {
    ArgumentError,
    RedefFunc,
    Type,
    UndefFunc,
    UndefVar,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorType::ArgumentError => write!(fmt, "ArgumentError"),
            ErrorType::RedefFunc => write!(fmt, "RedefFunc"),
            ErrorType::Type => write!(fmt, "TypeError"),
            ErrorType::UndefFunc => write!(fmt, "UndefFunc"),
            ErrorType::UndefVar => write!(fmt, "UndefVarError"),
        }
    }
}
