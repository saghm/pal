use std::collections::HashMap;

use ast::Value;

#[derive(Clone)]
pub struct Scope {
    map: HashMap<String, Value>,
    pub parent: Option<Box<Scope>>,
}

impl Scope {
    pub fn new(parent: Option<Self>) -> Self {
        Scope { map: HashMap::new(), parent: parent.map(Box::new) }
    }

    pub fn assign(&mut self, var: &str, val: Value) {
        self.map.insert(String::from(var), val);
    }

    pub fn lookup(&self, var: &str) -> Option<&Value> {
        self.map.get(var)
    }

    pub fn contains_var(&self, var: &str) -> bool {
        self.map.contains_key(var)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn parent(&self) -> Option<&Box<Scope>> {
        self.parent.as_ref()
    }
}
