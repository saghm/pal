#[cfg(test)]
mod test;

mod bin_exp;

use ast::{BinOp, Expr, Statement, Value};
use self::bin_exp::{arith_exp, bool_exp, eq_exp, ineq_exp};
use error::{Error, Result};
use state::State;

impl Statement {
    pub fn eval(&self, state: &mut State) -> Result<Option<Value>> {
        match *self {
            Statement::Assign(ref var, ref exp) => {
                let val = try!(exp.eval(state));
                state.assign(var, val).map(|_| None)
            }
            Statement::Defun(ref t, ref name, ref params, ref body) =>
                state.define_func(t, name, params, body).map(|_| None),
            Statement::If(ref exp, ref block1, ref block2) => {
                let val = try!(exp.eval(state));
                let block = match val {
                    Value::Bool(true) => block1,
                    Value::Bool(false) => block2,
                    _ => return Err(Error::type_error(
                        &format!("`{}` is {}, so `if ({}) ...` doesn't make sense", exp, val.type_string_with_article(), exp))),
                };

                for stmt in block.iter() {
                    if let v @ Some(_) = try!(stmt.eval(state)) {
                        return Ok(v);
                    }
                }

                Ok(None)
            }
            Statement::Let(ref var, ref exp) => {
                let val = try!(exp.eval(state));
                state.define_var(var, val);
                Ok(None)
            }
            Statement::Print(ref exp) => {
                println!("{}", try!(exp.eval(state)));
                Ok(None)
            }
            Statement::Return(ref exp) => exp.eval(state).map(Some),
            Statement::VoidCall(ref name, ref args) => state.call_function(name, args).map(|_| None),
            Statement::While(ref exp, ref block) => {
                let val = try!(exp.eval(state));
                match val {
                    Value::Bool(true) => (),
                    Value::Bool(false) => return Ok(None),
                    _ => return Err(Error::type_error(
                        &format!("`{}` is {}, so `while ({}) ...` doesn't make sense", exp, val.type_string_with_article(), exp))),
                };

                for stmt in block.iter() {
                    if let v @ Some(_) = try!(stmt.eval(state)) {
                        return Ok(v);
                    }
                }

                self.eval(state)
            }
        }
    }
}

impl Expr {
    pub fn eval(&self, state: &mut State) -> Result<Value> {
        match *self {
            Expr::BinExp(ref exp1, ref op, ref exp2) => {
                let val1 = try!(exp1.eval(state));
                let val2 = try!(exp2.eval(state));

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
                }
            }
            Expr::Call(ref name, ref args) => {
                match state.call_function(name, args) {
                    Ok(Some(val)) => Ok(val),
                    Ok(None) => Err(Error::type_error(
                        &format!("The function {} doesn't return anything, so {} doesn't make sense", name, self))),
                    Err(e) => Err(e),
                }
            }
            Expr::Not(ref exp) => {
                match try!(exp.eval(state)) {
                    Value::Bool(b) => Ok(Value::Bool(!b)),
                    _ => Err(Error::type_error(
                        &format!("`{}` is not a boolean, so `!{}` doesn't make sense", exp, exp))),
                }
            }
            Expr::Value(ref val) => Ok(val.clone()),
            Expr::Var(ref var) => {
                match state.lookup(var) {
                    Some(val) => Ok(val.clone()),
                    None => Err(Error::undef_var_error(
                        &format!("The variable `{}` is not defined, so it can't be used in an expression", var)))
                }
            }
        }
    }
}
