use std::collections::HashMap;

use ast::Value;

#[derive(Clone)]
pub struct Scope {
    // Maps variable names to values.
    map: HashMap<String, Value>,
    // The scope containing this one.
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new(parent: Option<Self>) -> Self {
        Scope { map: HashMap::new(), parent: parent.map(Box::new) }
    }

    // Assign a value to a given variable.
    pub fn assign(&mut self, var: &str, val: Value) {
        self.map.insert(String::from(var), val);
    }

    // Lookup the value associated with a given variable.
    pub fn lookup(&self, var: &str) -> Option<&Value> {
        self.map.get(var)
    }

    // Returns whether the variable is defined int the scope.
    pub fn contains_var(&self, var: &str) -> bool {
        self.map.contains_key(var)
    }

    // Returns number of variables in the scope.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    // Returns a reference to the containing scope.
    pub fn parent(&self) -> Option<&Box<Scope>> {
        self.parent.as_ref()
    }
}
