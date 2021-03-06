#[cfg(test)]
mod test;

mod bin_exp;

use std::io::{self, Write};
use std::sync::Arc;

use ast::{BinOp, Expr, Statement, Value};
use self::bin_exp::{arith_exp, bool_exp, eq_exp, ineq_exp};
use error::{Error, Result};
use state::State;
use stream::Stream;

use stepper::Stepper;

impl Statement {
    pub fn eval(&self, state: &mut State, stream_opt: Option<Arc<Stream>>) -> Result<Option<Value>> {
        match *self {
            Statement::ArrayElemAssign(ref var, ref index, ref indexes, ref exp) => {
                let mut array_vec = match state.lookup(var) {
                    Some(&Value::Array(ref vec)) => vec.clone(),
                    Some(ref val) => return Error::type_error(
                        &format!("`{}` is {}, so `{}` doesn't make sense", var, val.type_string_with_article(), self)),
                    None => return Error::undef_var_error(
                        &format!("The variable `{}` is not defined, so {} doesn't make sense", var, self)),
                };

                let index_val = try!(index.eval(state, stream_opt.clone()));
                let mut index_int = match index_val {
                    Value::Int(i) => i,
                    _ => return Error::type_error(
                        &format!("`{}` is {}, so `{}` doesn't make sense", var, index_val.type_string_with_article(), self))
                };

                if index_int < 0 {
                    return Error::array_index_out_of_bounds_error(
                        &format!("`{}` is {}, so `{}` doesn't make sense",
                        index, index_int, self))
                }

                if index_int as usize >= array_vec.len() {
                    return Error::array_index_out_of_bounds_error(
                        &format!("`{}` has {} elements in it, so `{}` doesn't make sense",
                        var, array_vec.len(), self))
                }

                let mut repr = format!("{}[{}]", var, index);

                for idx in indexes {
                    repr.push_str(&format!("[{}]", idx));

                    array_vec = match array_vec[index_int as usize] {
                        Value::Array(ref vec) => vec.clone(),
                        ref val => return Error::type_error(
                            &format!("`{}` is {}, so `{}` doesn't make sense", repr, val.type_string_with_article(), self)),
                    };

                    let index_val = try!(idx.eval(state, stream_opt.clone()));

                    index_int = match index_val {
                        Value::Int(i) => i,
                        _ => return Error::type_error(
                            &format!("`{}` is {}, so `{}` doesn't make sense", var, index_val.type_string_with_article(), self))
                    };

                    if index_int < 0 {
                        return Error::array_index_out_of_bounds_error(
                            &format!("`{}` is {}, so `{}` doesn't make sense",
                            index, index_int, self))
                    }

                    if index_int as usize >= array_vec.len() {
                        return Error::array_index_out_of_bounds_error(
                            &format!("`{}` has {} elements in it, so `{}` doesn't make sense",
                            var, array_vec.len(), self))
                    }
                }

                let exp_val = try!(exp.eval(state, stream_opt));
                array_vec[index_int as usize] = exp_val;
                state.assign(var, Value::Array(array_vec)).map(|_| None)
            }
            Statement::Delete(ref var, ref index, ref indexes) => {
                let mut array_vec = match state.lookup(var) {
                    Some(&Value::Array(ref vec)) => vec.clone(),
                    Some(ref val) => return Error::type_error(
                        &format!("`{}` is {}, so `{}` doesn't make sense", var, val.type_string_with_article(), self)),
                    None => return Error::undef_var_error(
                        &format!("The variable `{}` is not defined, so {} doesn't make sense", var, self)),
                };

                let index_val = try!(index.eval(state, stream_opt.clone()));
                let mut index_int = match index_val {
                    Value::Int(i) => i,
                    _ => return Error::type_error(
                        &format!("`{}` is {}, so `{}` doesn't make sense", var, index_val.type_string_with_article(), self))
                };

                if index_int < 0 {
                    return Error::array_index_out_of_bounds_error(
                        &format!("`{}` is {}, so `{}` doesn't make sense",
                        index, index_int, self))
                }

                if index_int as usize >= array_vec.len() {
                    return Error::array_index_out_of_bounds_error(
                        &format!("`{}` has {} elements in it, so `{}` doesn't make sense",
                        var, array_vec.len(), self))
                }

                let mut repr = format!("{}[{}]", var, index);
                let mut curr = &mut array_vec as *mut Vec<Value>;

                for idx in indexes {
                    repr.push_str(&format!("[{}]", idx));

                    unsafe {
                        curr = match (*curr)[index_int as usize] {
                            Value::Array(ref mut vec) => vec as *mut Vec<Value>,
                            ref val => return Error::type_error(
                                &format!("`{}` is {}, so `{}` doesn't make sense", repr, val.type_string_with_article(), self)),
                        };
                    }

                    let index_val = try!(idx.eval(state, stream_opt.clone()));

                    index_int = match index_val {
                        Value::Int(i) => i,
                        _ => return Error::type_error(
                            &format!("`{}` is {}, so `{}` doesn't make sense", var, index_val.type_string_with_article(), self))
                    };

                    if index_int < 0 {
                        return Error::array_index_out_of_bounds_error(
                            &format!("`{}` is {}, so `{}` doesn't make sense",
                            index, index_int, self))
                    }

                    let curr_len = unsafe { (*curr).len() };

                    if index_int as usize >= curr_len {
                        return Error::array_index_out_of_bounds_error(
                            &format!("`{}` has {} elements in it, so `{}` doesn't make sense",
                            var, array_vec.len(), self))
                    }

                }

                unsafe {
                    (*curr).remove(index_int as usize);
                }
                try!(state.assign(var, Value::Array(array_vec)));
                Ok(None)
            }
            Statement::Defun(ref t, ref name, ref params, ref body) =>
                state.define_func(t, name, params, body).map(|_| None),
            Statement::For(ref var, ref exp, ref block) => {
                let val = try!(exp.eval(state, stream_opt.clone()));
                let vec = match val {
                    Value::Array(vec) => vec,
                    _ => return Error::type_error(
                        &format!("`{}` is {}, so `for {} in {} ...` doesn't make sense", exp, val.type_string_with_article(), var, exp)),
                };

                for array_val in vec {
                    state.define_var(var, array_val);

                    for stmt in block {
                        try!(stmt.eval(state, stream_opt.clone()));
                    }
                }

                Ok(None)
            }
            Statement::If(ref exp, ref block1, ref block2) => {
                let val = try!(exp.eval(state, stream_opt.clone()));
                let block = match val {
                    Value::Bool(true) => block1,
                    Value::Bool(false) => block2,
                    _ => return Error::type_error(
                        &format!("`{}` is {}, so `if ({}) ...` doesn't make sense", exp, val.type_string_with_article(), exp)),
                };

                for stmt in block.iter() {
                    if let v @ Some(_) = try!(stmt.eval(state, stream_opt.clone())) {
                        return Ok(v);
                    }
                }

                Ok(None)
            }
            Statement::Let(ref var, ref exp) => {
                let val = try!(exp.eval(state, stream_opt));
                state.define_var(var, val);
                Ok(None)
            }
            Statement::Print(ref exp) => match stream_opt.clone() {
                Some(stream) => {
                    stream.write_output(&format!("{}", try!(exp.eval(state, stream_opt))));
                    Ok(None)
                }
                None => {
                    print!("{}", try!(exp.eval(state, stream_opt)));
                    io::stdout().flush().unwrap();
                    Ok(None)
                }
            },
            Statement::PrintLine(ref exp) => match stream_opt.clone() {
                Some(stream) => {
                    stream.write_output(&format!("{}\n", try!(exp.eval(state, stream_opt))));
                    Ok(None)
                }
                None => {
                    println!("{}", try!(exp.eval(state, stream_opt)));
                    io::stdout().flush().unwrap();
                    Ok(None)
                }
            },
            Statement::Return(ref exp) => exp.eval(state, stream_opt).map(Some),
            Statement::VarAssign(ref var, ref exp) => {
                let val = try!(exp.eval(state, stream_opt));
                state.assign(var, val).map(|_| None)
            }
            Statement::VoidCall(ref name, ref args) => state.call_function(name, args, stream_opt).map(|_| None),
            Statement::While(ref exp, ref block) => {

                loop {
                    let val = try!(exp.eval(state, stream_opt.clone()));

                    match val {
                        Value::Bool(true) => (),
                        Value::Bool(false) => return Ok(None),
                        _ => return Error::type_error(
                            &format!("`{}` is {}, so `while ({}) ...` doesn't make sense", exp, val.type_string_with_article(), exp)),
                    };

                    for stmt in block.iter() {
                        if let v @ Some(_) = try!(stmt.eval(state, stream_opt.clone())) {
                            return Ok(v);
                        }
                    }
                }
            }
        }
    }
}

impl Expr {
    pub fn eval(&self, state: &mut State, stream_opt: Option<Arc<Stream>>) -> Result<Value> {
        match *self {
            Expr::Array(ref vec) => {
                let mut out = Vec::new();

                for ref exp in vec {
                    out.push(try!(exp.eval(state, stream_opt.clone())));
                }

                Ok(Value::Array(out))
            }
            Expr::ArrayElement(ref var, ref index, ref indexes) => {
                let mut array_vec = match state.lookup(var) {
                    Some(&Value::Array(ref vec)) => vec.clone(),
                    Some(ref val) => return Error::type_error(
                        &format!("`{}` is {}, so `{}` doesn't make sense", var, val.type_string_with_article(), self)),
                    None => return Error::undef_var_error(
                        &format!("The variable `{}` is not defined, so {} doesn't make sense", var, self)),
                };

                let index_val = try!(index.eval(state, stream_opt.clone()));
                let mut index_int = match index_val {
                    Value::Int(i) => i,
                    _ => return Error::type_error(
                        &format!("`{}` is {}, so `{}` doesn't make sense", var, index_val.type_string_with_article(), self))
                };

                if index_int < 0 {
                    return Error::array_index_out_of_bounds_error(
                        &format!("`{}` is {}, so `{}` doesn't make sense",
                        index, index_int, self))
                }

                if index_int as usize >= array_vec.len() {
                    return Error::array_index_out_of_bounds_error(
                        &format!("`{}` has {} elements in it, so `{}` doesn't make sense",
                        var, array_vec.len(), self))
                }

                let mut repr = format!("{}[{}]", var, index);

                for idx in indexes {
                    repr.push_str(&format!("[{}]", idx));

                    array_vec = match array_vec[index_int as usize] {
                        Value::Array(ref vec) => vec.clone(),
                        ref val => return Error::type_error(
                            &format!("`{}` is {}, so `{}` doesn't make sense", repr, val.type_string_with_article(), self)),
                    };

                    let index_val = try!(idx.eval(state, stream_opt.clone()));

                    index_int = match index_val {
                        Value::Int(i) => i,
                        _ => return Error::type_error(
                            &format!("`{}` is {}, so `{}` doesn't make sense", var, index_val.type_string_with_article(), self))
                    };

                    if index_int < 0 {
                        return Error::array_index_out_of_bounds_error(
                            &format!("`{}` is {}, so `{}` doesn't make sense",
                            index, index_int, self))
                    }

                    if index_int as usize >= array_vec.len() {
                        return Error::array_index_out_of_bounds_error(
                            &format!("`{}` has {} elements in it, so `{}` doesn't make sense",
                            var, array_vec.len(), self))
                    }
                }

                Ok(array_vec[index_int as usize].clone())
            }
            Expr::BinExp(ref exp1, ref op, ref exp2) => {
                let val1 = try!(exp1.eval(state, stream_opt.clone()));
                let val2 = try!(exp2.eval(state, stream_opt));

                match *op {
                    BinOp::And => bool_exp(self, val1, val2, |x, y| x && y),
                    BinOp::Or =>  bool_exp(self, val1, val2, |x, y| x || y),
                    BinOp::Equal => eq_exp(self, val1, val2, |x, y| x == y),
                    BinOp::NotEqual => eq_exp(self, val1, val2, |x, y| x != y),
                    BinOp::GreaterOrEqual => ineq_exp(self, val1, val2, |x, y| x >= y),
                    BinOp::GreaterThan => ineq_exp(self, val1, val2, |x, y| x > y),
                    BinOp::LessOrEqual => ineq_exp(self, val1, val2, |x, y| x <= y),
                    BinOp::LessThan => ineq_exp(self, val1, val2, |x, y| x < y),
                    BinOp::Plus => arith_exp(self, val1, val2, |x, y| x + y),
                    BinOp::Minus => arith_exp(self, val1, val2, |x, y| x - y),
                    BinOp::Times => arith_exp(self, val1, val2, |x, y| x * y),
                    BinOp::Divide => arith_exp(self, val1, val2, |x, y| x / y),
                    BinOp::Modulus => arith_exp(self, val1, val2, |x, y| x % y),
                    BinOp::Concat => match (val1, val2) {
                        (Value::Array(mut v1), Value::Array(v2)) => {
                            v1.extend(v2);
                            Ok(Value::Array(v1))
                        }
                        (Value::Array(_), v2) => Error::type_error(
                            &format!("`{}` is not an array, so `{}` is invalid", v2, self)),
                        (v1, _) => Error::type_error(
                            &format!("`{}` is not an array, so `{}` is invalid", v1, self)),
                    },
                }
            }
            Expr::Call(ref name, ref args) => {
                match state.call_function(name, args, stream_opt) {
                    Ok(Some(val)) => Ok(val),
                    Ok(None) => Error::type_error(
                        &format!("The function {} doesn't return anything, so {} doesn't make sense", name, self)),
                    Err(e) => Err(e),
                }
            }
            Expr::Length(ref exp) => {
                let val = try!(exp.eval(state, stream_opt));

                match val {
                    Value::Array(ref vec) => Ok(Value::Int(vec.len() as i64)),
                    Value::Str(ref string) => Ok(Value::Int(string.len() as i64)),
                    _ => Error::type_error(
                        &format!("{} is {}, so {} doesn't make sense", exp, val.type_string_with_article(), self))
                }
            }
            Expr::Letters(ref exp) => {
                let val = try!(exp.eval(state, stream_opt));

                match val {
                    Value::Str(ref string) => Ok(Value::Array(string.chars().map(|c| Value::Str(format!("{}", c))).collect())),
                    _ => Error::type_error(
                        &format!("{} is {}, so {} doesn't make sense", exp, val.type_string_with_article(), self))
                }
            }
            Expr::Not(ref exp) => {
                match try!(exp.eval(state, stream_opt)) {
                    Value::Bool(b) => Ok(Value::Bool(!b)),
                    _ => Error::type_error(
                        &format!("`{}` is not a boolean, so `!{}` doesn't make sense", exp, exp)),
                }
            }
            Expr::Range(ref start, ref end) => {
                let start_int = match try!(start.eval(state, stream_opt.clone())) {
                    Value::Int(i) => i,
                    _ => return Error::type_error(
                        &format!("`{}` is not a int, so `{}` doesn't make sense", start, self)),
                };

                let end_int = match try!(end.eval(state, stream_opt)) {
                    Value::Int(i) => i,
                    _ => return Error::type_error(
                        &format!("`{}` is not a int, so `{}` doesn't make sense", end, self)),
                };

                // `stepper` ranges are not end-inclusive.
                let (fixed_end, step_int) = if end_int >= start_int {
                    (end_int + 1, 1)
                } else {
                    (end_int - 1, -1)
                };

                let vec: Vec<_> = step!(start_int => fixed_end; step_int).into_iter().map(Value::Int).collect();
                Ok(Value::Array(vec))
            }
            Expr::ReadLine => match stream_opt.clone() {
                Some(stream) => Ok(Value::Str(stream.read_input())),
                None => {
                    let mut buf = String::new();
                    io::stdin().read_line(&mut buf).unwrap();

                    if let Some('\n') = buf.chars().last() {
                        let _ = buf.pop();
                    }

                    Ok(Value::Str(buf))
                }
            },
            Expr::Step(ref start, ref end, ref step) => {
                let start_int = match try!(start.eval(state, stream_opt.clone())) {
                    Value::Int(i) => i,
                    _ => return Error::type_error(
                        &format!("`{}` is not a int, so `{}` doesn't make sense", start, self)),
                };

                let end_int = match try!(end.eval(state, stream_opt.clone())) {
                    Value::Int(i) => i,
                    _ => return Error::type_error(
                        &format!("`{}` is not a int, so `{}` doesn't make sense", end, self)),
                };

                let step_int = match try!(step.eval(state, stream_opt)) {
                    Value::Int(i) => i,
                    _ => return Error::type_error(
                        &format!("`{}` is not a int, so `{}` doesn't make sense", step, self)),
                };

                if start_int < end_int && step_int < 0 {
                    return Error::step_error(
                        &format!("`{}` = {}, `{}` = {}, and `{}` = {}; stepping down from {} will never reach {}",
                            start, start_int, end, end_int, step, step_int, start_int, end_int));
                }

                if start_int > end_int && step_int > 0 {
                    return Error::step_error(
                        &format!("`{}` = {}, `{}` = {}, and `{}` = {}; stepping up from {} will never reach {}",
                            start, start_int, end, end_int, step, step_int, start_int, end_int));
                }

                if step_int == 0 {
                    return Error::step_error(
                        &format!("`{}` = {}, `{}` = {}, and `{}` = {}; stepping by 0 will never get anywhere",
                            start, start_int, end, end_int, step, step_int));
                }

                // `stepper` ranges are not end-inclusive.
                let fixed_end = end_int + step_int.signum();

                let vec: Vec<_> = step!(start_int => fixed_end; step_int).into_iter().map(Value::Int).collect();
                Ok(Value::Array(vec))
            }
            Expr::Value(ref val) => Ok(val.clone()),
            Expr::Var(ref var) => {
                match state.lookup(var) {
                    Some(val) => Ok(val.clone()),
                    None => Error::undef_var_error(
                        &format!("The variable `{}` is not defined, so it can't be used in an expression", var))
                }
            }
        }
    }
}
