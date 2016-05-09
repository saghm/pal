use ast::{Expr, Value};
use error::{Error, Result};

pub fn bool_exp<F>(exp: &Expr, v1: Value, v2: Value, fun: F) -> Result<Value>
    where F: Fn(bool, bool) -> bool {
    match (&v1, &v2) {
        (&Value::Bool(b1), &Value::Bool(b2)) => Ok(Value::Bool(fun(b1, b2))),
        (&Value::Bool(_), _) => Err(Error::type_error(
            &format!("`{}` is not a boolean, so `{}` is invalid", v2, exp))),
        _ => Err(Error::type_error(
            &format!("`{}` is not a boolean, so `{}` is invalid", v1, exp))),
    }
}

pub fn eq_exp<F>(exp: &Expr, v1: Value, v2: Value, fun: F) -> Result<Value>
    where F: Fn(Value, Value) -> bool {
    match (&v1, &v2) {
        (&Value::Bool(_), &Value::Bool(_)) |
        (&Value::Int(_), &Value::Int(_)) => Ok(Value::Bool(fun(v1, v2))),
        (&Value::Bool(_), &Value::Int(_)) => Err(Error::type_error(
            &format!("`{}` is a boolean and `{}` is an int, so `{}` is invalid", v1, v2, exp))),
        (&Value::Bool(_), &Value::Str(_)) => Err(Error::type_error(
            &format!("`{}` is a boolean and `{}` is a string, so `{}` is invalid", v1, v2, exp))),
        (&Value::Int(_), &Value::Bool(_)) => Err(Error::type_error(
            &format!("`{}` is an int and `{}` is a boolean, so `{}` is invalid", v1, v2, exp))),
        (&Value::Int(_), &Value::Str(_)) => Err(Error::type_error(
            &format!("`{}` is an int and `{}` is a string, so `{}` is invalid", v1, v2, exp))),
        (&Value::Str(_), &Value::Bool(_)) => Err(Error::type_error(
            &format!("`{}` is a string and `{}` is a boolean, so `{}` is invalid", v1, v2, exp))),
        (&Value::Str(_), &Value::Int(_)) => Err(Error::type_error(
            &format!("`{}` is a string and `{}` is a int, so `{}` is invalid", v1, v2, exp))),
        (&Value::Str(_), &Value::Str(_)) => Err(Error::type_error(
            &format!("`{}` is a string and `{}` is a string, so `{}` is invalid", v1, v2, exp))),
    }
}


pub fn ineq_exp<F>(exp: &Expr, v1: Value, v2: Value, fun: F) -> Result<Value>
    where F: Fn(i64, i64) -> bool {
    match (&v1, &v2) {
        (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Bool(fun(i1, i2))),
        (&Value::Int(_), _) => Err(Error::type_error(
            &format!("`{}` is not an int, so `{}` is invalid", v2, exp))),
        _ => Err(Error::type_error(
            &format!("`{}` is not an int, so `{}` is invalid", v1, exp))),

    }
}

pub fn arith_exp<F>(exp: &Expr, v1: Value, v2: Value, fun: F) -> Result<Value>
    where F: Fn(i64, i64) -> i64 {
    match (&v1, &v2) {
        (&Value::Int(i1), &Value::Int(i2)) => Ok(Value::Int(fun(i1, i2))),
        (&Value::Int(_), _) => Err(Error::type_error(
            &format!("`{}` is not an int, so `{}` is invalid", v2, exp))),
        _ => Err(Error::type_error(
            &format!("`{}` is not an int, so `{}` is invalid", v1, exp))),

    }
}
