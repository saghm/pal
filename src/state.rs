use std::collections::HashMap;

use ast::Value;

#[derive(Clone)]
struct Scope {
    map: HashMap<String, Value>,
    parent: Option<Box<Scope>>,
}

impl Scope {
    fn new(parent: Option<Self>) -> Self {
        Scope { map: HashMap::new(), parent: parent.map(Box::new) }
    }

    fn assign(&mut self, var: &str, val: Value) {
        self.map.insert(String::from(var), val);
    }

    fn lookup(&self, var: &str) -> Option<&Value> {
        self.map.get(var)
    }

    fn contains_var(&self, var: &str) -> bool {
        self.map.contains_key(var)
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn parent(&self) -> Option<&Box<Scope>> {
        self.parent.as_ref()
    }
}

pub struct State {
    global: Scope,
    current: Option<Scope>,
}

impl State {
    pub fn new() -> Self {
        State { global: Scope::new(None), current: None }
    }

    pub fn assign(&mut self, var: &str, val: Value) {
        if let Some(ref mut scope) = self.current {
            return scope.assign(var, val);
        }

        self.global.assign(var, val);
    }

    pub fn contains_var(&self, var: &str) -> bool {
        self.current.as_ref().map(|s| s.contains_var(var)).unwrap_or(false) || self.global.contains_var(var)
    }

    pub fn enter_scope(&mut self) {
        self.current = Some(Scope::new(self.current.clone()));
    }

    pub fn exit_scope(&mut self) {
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
        let mut temp = match &self.current {
            &Some(ref s) => {
                len += s.len();
                s.parent()
            }
            &None => return len,
        };

        while let Some(scope) = temp {
            len += scope.len();
            temp = scope.parent();
        }

        len
    }
}
