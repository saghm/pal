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

    #[inline]
    #[allow(dead_code)]
    pub fn err_type(&self) -> ErrorType {
        self.err.clone()
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

    pub fn step_error<T>(s: &str) -> Result<T> {
        Err(Self::new(ErrorType::Step, s))
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorType {
    Argument,
    ArrayIndexOutOfBounds,
    RedefinedFunction,
    Step,
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
            ErrorType::Step => write!(fmt, "StepError"),
            ErrorType::Type => write!(fmt, "TypeError"),
            ErrorType::UndefinedFunction => write!(fmt, "UndefinedFunctionError"),
            ErrorType::UndefinedVariable => write!(fmt, "UndefinedVariableError"),
        }
    }
}
