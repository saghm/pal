mod function;
mod scope;

use std::collections::HashMap;

use self::function::Function;
use self::scope::Scope;

use ast::{Expr, Statement, Type, Value};
use error::{Error, Result};

pub struct State {
    global: Scope,
    current: Option<Scope>,
    functions: HashMap<String, Function>,
}

impl State {
    pub fn new() -> Self {
        State { global: Scope::new(None), current: None, functions: HashMap::new() }
    }

    pub fn assign(&mut self, var: &str, val: Value) -> Result<()> {
        if let Some(ref mut scope) = self.current {
            if scope.contains_var(var) {
                return Ok(scope.assign(var, val));
            }
        }

        if self.global.contains_var(var) {
            return Ok(self.global.assign(var, val));
        }

        Err(Error::undef_var_error(
            &format!("The variable `{}` has not been declared, so it can't have a value assigned to it", var)))
    }

    pub fn call_function(&mut self, name: &str, args: &[Expr]) -> Result<Option<Value>> {
        macro_rules! try_or_exit_scope {
            ($e:expr, $state:expr) => { match $e {
                Ok(t) => t,
                e @ Err(_) => {
                    $state.exit_scope();
                    return e;
                }
            }}
        }

        let (return_type, params, body) = match self.functions.get(name) {
            Some(&Function { ref return_type, ref params, ref body }) =>
                (return_type.clone(), params.clone(), body.clone()),
            None => return Err(Error::undef_func_error(
                &format!("The function `{}` has not been defined, so it can't be called", name)))
        };

        if params.len() != args.len() {
            return Err(Error::argument_error(
                &format!("The function {} takes {} arguments, but {} were given", name, params.len(), args.len())));
        }

        let mut arg_values = Vec::new();

        for arg in args.iter() {
            arg_values.push(try!(arg.eval(self)));
        }

        self.enter_scope();

        for (i, val) in arg_values.into_iter().enumerate() {
            self.define_var(&params[i], val);
        }

        for stmt in &body {
            if let Some(val) = try_or_exit_scope!(stmt.eval(self), self) {
                self.exit_scope();
                return if val.is_a(&return_type) {
                    Ok(Some(val))
                } else {
                    Err(Error::type_error(
                        &format!("The function {} is supposed to return {}, but instead it returns {}",
                            name, return_type.as_string_with_article(), val.type_string_with_article())))
                };
            }
        }

        self.exit_scope();

        if return_type != Type::Void {
            Err(Error::type_error(
                &format!("The function {} is supposed to return {}, but instead it returns nothing",
                    name, return_type.as_string_with_article())))
        } else {
            Ok(None)
        }
    }

    pub fn define_func(&mut self, return_type: &Type, name: &str, params: &[String], body: &[Statement]) -> Result<()> {
        if self.functions.contains_key(name) {
            return Err(Error::redef_func_error(
                &format!("The function {} has already been defined, so it can't be defined again", name)));
        }

        self.functions.insert(String::from(name), Function::new(return_type, params, body));
        Ok(())
    }

    pub fn define_var(&mut self, var: &str, val: Value) {
        if let Some(ref mut scope) = self.current {
            return scope.assign(var, val);
        }

        self.global.assign(var, val);
    }

    fn enter_scope(&mut self) {
        self.current = Some(Scope::new(self.current.clone()));
    }

    fn exit_scope(&mut self) {
        let new = match self.current {
            Some(ref scope) => scope.parent.clone().map(|s| *s),
            None => None,
        };

        self.current = new;
    }

    pub fn lookup(&self, var: &str) -> Option<&Value> {
        if let Some(ref scope) = self.current {
            if let Some(ref val) = scope.lookup(var) {
                return Some(val);
            }
        }

        self.global.lookup(var)
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        let mut len = self.global.len();
        let mut temp = match self.current {
            Some(ref s) => {
                len += s.len();
                s.parent()
            }
            None => return len,
        };

        while let Some(scope) = temp {
            len += scope.len();
            temp = scope.parent();
        }

        len
    }
}
