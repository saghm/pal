#[cfg(test)]
mod test;

mod bin_exp;

use ast::{BinOp, Expr, Statement, Value};
use self::bin_exp::{arith_exp, bool_exp, eq_exp, ineq_exp};
use error::{Error, Result};
use state::State;

impl Statement {
    pub fn eval(&self, state: &mut State) -> Result<()> {
        match *self {
            Statement::Assign(ref var, ref e) => {
                if !state.contains_key(var) {
                    return Err(Error::undef_var_error(
                        &format!("The variable `{}` has not been declared, so it can't have a value assigned to it", var)))
                }

                let v = try!(e.eval(state));
                state.insert(var.clone(), v);
                Ok(())
            }
            Statement::If(ref exp, ref block1, ref block2) => {
                let block = match try!(exp.eval(state)) {
                    Value::Bool(true) => block1,
                    Value::Bool(false) => block2,
                    Value::Int(_) => return Err(Error::type_error(
                        &format!("`{}` is an int, so `if ({}) ...` is invalid", exp, exp)))
                };

                for stmt in block.iter() {
                    try!(stmt.eval(state));
                }

                Ok(())
            }
            Statement::Let(ref var, ref e) => {
                let v = try!(e.eval(state));
                state.insert(var.clone(), v);
                Ok(())
            }
            Statement::Print(ref e) => {
                println!("{}", try!(e.eval(state)));
                Ok(())
            }
        }
    }
}

impl Expr {
    pub fn eval(&self, state: &mut State) -> Result<Value> {
        match *self {
            Expr::BinExp(ref e1, ref o, ref e2) => {
                let v1 = try!(e1.eval(state));
                let v2 = try!(e2.eval(state));

                match *o {
                    BinOp::And => bool_exp(self, v1, v2, |x, y| x && y),
                    BinOp::Or =>  bool_exp(self, v1, v2, |x, y| x || y),
                    BinOp::Equal => eq_exp(self, v1, v2, |x, y| x == y),
                    BinOp::NotEqual => eq_exp(self, v1, v2, |x, y| x != y),
                    BinOp::GreaterOrEqual => ineq_exp(self, v1, v2, |x, y| x >= y),
                    BinOp::GreaterThan => ineq_exp(self, v1, v2, |x, y| x > y),
                    BinOp::LessOrEqual => ineq_exp(self, v1, v2, |x, y| x <= y),
                    BinOp::LessThan => ineq_exp(self, v1, v2, |x, y| x < y),
                    BinOp::Plus => arith_exp(self, v1, v2, |x, y| x + y),
                    BinOp::Minus => arith_exp(self, v1, v2, |x, y| x - y),
                    BinOp::Times => arith_exp(self, v1, v2, |x, y| x * y),
                    BinOp::Divide => arith_exp(self, v1, v2, |x, y| x / y),
                    BinOp::Modulus => arith_exp(self, v1, v2, |x, y| x % y),
                }
            }
            Expr::Not(ref e) => {
                match try!(e.eval(state)) {
                    Value::Bool(b) => Ok(Value::Bool(!b)),
                    Value::Int(i) => Err(Error::type_error(
                        &format!("`{}` is not a boolean, so `!{}` is invalid", i, self))),
                }
            }
            Expr::Value(ref v) => Ok(v.clone()),
            Expr::Var(ref var) => {
                match state.get(var) {
                    Some(v) => Ok(v.clone()),
                    None => Err(Error::undef_var_error(
                        &format!("The variable `{}` is not defined, so it can't be used in an expression", var)))
                }
            }
        }
    }
}
