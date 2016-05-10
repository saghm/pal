use ast::{Statement, Type};

pub struct Function {
    pub return_type: Type,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
}

impl Function {
    pub fn new(return_type: &Type, params: &[String], body: &[Statement]) -> Self {
        Function { return_type: return_type.clone(), params: Vec::from(params), body: Vec::from(body) }
    }
}
