use ast::{Expr, Value};
use error::{Error, Result};

// Evaluate a binary expression on two boolean values yielding a boolean.
pub fn bool_exp<F>(exp: &Expr, val1: Value, val2: Value, func: F) -> Result<Value>
    where F: Fn(bool, bool) -> bool {
    match (&val1, &val2) {
        (&Value::Bool(b1), &Value::Bool(b2)) => Ok(Value::Bool(func(b1, b2))),
        (&Value::Bool(_), _) => Error::type_error(
            &format!("`{}` is not a boolean, so `{}` is invalid", val2, exp)),
        _ => Error::type_error(
            &format!("`{}` is not a boolean, so `{}` is invalid", val1, exp)),
    }
}

// Evaluate a binary expression on two same-typed values yielding a boolean.
pub fn eq_exp<F>(exp: &Expr, val1: Value, val2: Value, func: F) -> Result<Value>
    where F: Fn(Value, Value) -> bool {
    match (&val1, &val2) {
        (&Value::Bool(_), &Value::Bool(_)) |
        (&Value::Int(_), &Value::Int(_)) |
        (&Value::Str(_), &Value::Str(_)) => Ok(Value::Bool(func(val1, val2))),
        _ => Error::type_error(
            &format!("`{}` is {} and `{}` is {}, so `{}` doesn't make sense",
                     val1, val1.type_string_with_article(), val2, val2.type_string_with_article(), exp)),
    }
}

// Evaluate a binary expression on two int values yielding a boolean.
pub fn ineq_exp<F>(exp: &Expr, val1: Value, val2: Value, func: F) -> Result<Value>
    where F: Fn(i64, i64) -> bool {
    match (&val1, &val2) {
        (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Bool(func(i1, i2))),
        (&Value::Int(_), _) => Error::type_error(
            &format!("`{}` is not an int, so `{}` is invalid", val2, exp)),
        _ => Error::type_error(
            &format!("`{}` is not an int, so `{}` is invalid", val1, exp)),

    }
}

// Evaluate a binary expression on two int values yielding an int.
pub fn arith_exp<F>(exp: &Expr, val1: Value, val2: Value, func: F) -> Result<Value>
    where F: Fn(i64, i64) -> i64 {
    match (&val1, &val2) {
        (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(func(i1, i2))),
        (&Value::Int(_), _) => Error::type_error(
            &format!("`{}` is not an int, so `{}` is invalid", val2, exp)),
        _ => Error::type_error(
            &format!("`{}` is not an int, so `{}` is invalid", val1, exp)),

    }
}
