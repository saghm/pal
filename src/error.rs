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

    pub fn argument_error<T>(s: &str) -> Result<T> {
        Err(Self::new(ErrorType::Argument, s))
    }

    pub fn array_index_out_of_bounds_error<T>(s: &str) -> Result<T> {
        Err(Self::new(ErrorType::ArrayIndexOutOfBounds, s))
    }

    pub fn redef_func_error<T>(s: &str) -> Result<T> {
        Err(Self::new(ErrorType::RedefinedFunction, s))
    }

    pub fn type_error<T>(s: &str) -> Result<T> {
        Err(Self::new(ErrorType::Type, s))
    }

    pub fn undef_func_error<T>(s: &str) -> Result<T> {
        Err(Self::new(ErrorType::UndefinedFunction, s))
    }

    pub fn undef_var_error<T>(s: &str) -> Result<T> {
        Err(Self::new(ErrorType::UndefinedVariable, s))
    }
}

#[derive(Debug)]
pub enum ErrorType {
    Argument,
    ArrayIndexOutOfBounds,
    RedefinedFunction,
    Type,
    UndefinedFunction,
    UndefinedVariable,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorType::Argument => write!(fmt, "ArgumentError"),
            ErrorType::ArrayIndexOutOfBounds => write!(fmt, "ArrayIndexOutOfBoundsError"),
            ErrorType::RedefinedFunction => write!(fmt, "RedefinedFunctionError"),
            ErrorType::Type => write!(fmt, "TypeError"),
            ErrorType::UndefinedFunction => write!(fmt, "UndefinedFunctionError"),
            ErrorType::UndefinedVariable => write!(fmt, "UndefinedVariableError"),
        }
    }
}
