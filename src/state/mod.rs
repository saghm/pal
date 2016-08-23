mod function;
mod scope;

use std::collections::HashMap;
use std::sync::Arc;

use self::function::Function;
use self::scope::Scope;

use ast::{Expr, Statement, Type, Value};
use error::{Error, Result};
use stream::Stream;

pub struct State {
    // The global scope
    global: Scope,
    // The current evaluation scope
    current: Option<Scope>,
    // Maps function names to functions.
    functions: HashMap<String, Function>,
}

impl State {
    pub fn new() -> Self {
        State { global: Scope::new(None), current: None, functions: HashMap::new() }
    }

    // Assigns a value to a variable, returning an error if the variable is not already defined.
    pub fn assign(&mut self, var: &str, val: Value) -> Result<()> {
        // Check current scope
        if let Some(ref mut scope) = self.current {
            if scope.contains_var(var) {
                return Ok(scope.assign(var, val));
            }
        }

        // Check global scope
        if self.global.contains_var(var) {
            return Ok(self.global.assign(var, val));
        }

        Error::undef_var_error(
            &format!("The variable `{}` has not been declared, so it can't have a value assigned to it", var))
    }

    // Evaluates a function given its name and arguments.
    pub fn call_function(&mut self, name: &str, args: &[Expr], stream: Option<Arc<Stream>>) -> Result<Option<Value>> {
        macro_rules! try_or_exit_scope {
            ($e:expr, $state:expr) => { match $e {
                Ok(t) => t,
                e @ Err(_) => {
                    $state.exit_scope();
                    return e;
                }
            }}
        }

        // Look up the function
        let (return_type, params, body) = match self.functions.get(name) {
            Some(&Function { ref return_type, ref params, ref body }) =>
                (return_type.clone(), params.clone(), body.clone()),
            None => return Error::undef_func_error(
                &format!("The function `{}` has not been defined, so it can't be called", name))
        };

        // Check that the correct number of arguments is given
        if params.len() != args.len() {
            return Error::argument_error(
                &format!("The function {} takes {} arguments, but {} were given", name, params.len(), args.len()));
        }

        let mut arg_values = Vec::new();

        // Evaluate the arguments
        for arg in args.iter() {
            arg_values.push(try!(arg.eval(self, stream.clone())));
        }

        self.enter_scope();

        // Assign the argument values to the parameters
        for (i, val) in arg_values.into_iter().enumerate() {
            self.define_var(&params[i], val);
        }

        // Evaluate the function body
        for stmt in &body {
            // Check if the function has returned
            if let Some(val) = try_or_exit_scope!(stmt.eval(self, stream.clone()), self) {
                self.exit_scope();

                // Verify that the returned value matches the return type of the function
                return if val.is_a(&return_type) {
                    Ok(Some(val))
                } else {
                    Error::type_error(
                        &format!("The function {} is supposed to return {}, but instead it returns {}",
                            name, return_type.as_string_with_article(), val.type_string_with_article()))
                };
            }
        }

        self.exit_scope();

        // Verify that functions with a non-void return type have returned a value.
        if return_type != Type::Void {
            Error::type_error(
                &format!("The function {} is supposed to return {}, but instead it returns nothing",
                    name, return_type.as_string_with_article()))
        } else {
            Ok(None)
        }
    }

    // Defines a new function given its return type, name, parameters, and body, returning an error
    // if a function of that name is already defined.
    pub fn define_func(&mut self, return_type: &Type, name: &str, params: &[String], body: &[Statement]) -> Result<()> {
        if self.functions.contains_key(name) {
            return Error::redef_func_error(
                &format!("The function {} has already been defined, so it can't be defined again", name));
        }

        self.functions.insert(String::from(name), Function::new(return_type, params, body));
        Ok(())
    }

    // Defines a new variable in the current scope.
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

    // Looks up the value of a variable.
    pub fn lookup(&self, var: &str) -> Option<&Value> {
        // Check the current scope
        if let Some(ref scope) = self.current {
            if let Some(ref val) = scope.lookup(var) {
                return Some(val);
            }
        }

        // Check the global scope
        self.global.lookup(var)
    }

    // Returns the number of variables currently defined in the program (only used in the
    // evaluation test cases).
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        let mut len = self.global.len();

        // Check the current scope
        let mut temp = match self.current {
            Some(ref s) => {
                len += s.len();
                s.parent()
            }
            None => return len,
        };

        // Work through each parent scope until none is found.
        while let Some(scope) = temp {
            len += scope.len();
            temp = scope.parent();
        }

        len
    }
}
