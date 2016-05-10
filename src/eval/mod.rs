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
            Statement::Assign(ref var, ref exp) => {
                if !state.contains_var(var) {
                    return Err(Error::undef_var_error(
                        &format!("The variable `{}` has not been declared, so it can't have a value assigned to it", var)))
                }

                let val = try!(exp.eval(state));
                state.assign(var, val);
                Ok(())
            }
            Statement::Defun(..) => {
                unimplemented!()
            }
            Statement::If(ref exp, ref block1, ref block2) => {
                let val = try!(exp.eval(state));
                let block = match val {
                    Value::Bool(true) => block1,
                    Value::Bool(false) => block2,
                    _ => return Err(Error::type_error(
                        &format!("`{}` is {}, so `if ({}) ...` doesn't make sense", exp, val.type_string_with_article(), exp))),
                };

                for stmt in block.iter() {
                    try!(stmt.eval(state));
                }

                Ok(())
            }
            Statement::Let(ref var, ref exp) => {
                let val = try!(exp.eval(state));
                state.assign(var, val);
                Ok(())
            }
            Statement::Print(ref exp) => {
                println!("{}", try!(exp.eval(state)));
                Ok(())
            }
            Statement::While(ref exp, ref block) => {
                let val = try!(exp.eval(state));
                match val {
                    Value::Bool(true) => (),
                    Value::Bool(false) => return Ok(()),
                    _ => return Err(Error::type_error(
                        &format!("`{}` is {}, so `while ({}) ...` doesn't make sense", exp, val.type_string_with_article(), exp))),
                };

                for stmt in block.iter() {
                    try!(stmt.eval(state));
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
            Expr::Call(..) => {
                unimplemented!()
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
