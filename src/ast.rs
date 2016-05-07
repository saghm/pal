use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i64),
}

impl fmt::Display for Value {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Bool(b) => write!(fmt, "{}", b),
            Value::Int(i) => write!(fmt, "{}", i),
        }
    }
}

#[derive(Debug)]
pub enum BinOp {
    // Boolean
    And,
    Or,

    // Comparisons
    Equal,
    NotEqual,
    GreaterOrEqual,
    GreaterThan,
    LessOrEqual,
    LessThan,

    // Arithmetic
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
}

impl fmt::Display for BinOp {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BinOp::And => write!(fmt, "&&"),
            BinOp::Or => write!(fmt, "||"),
            BinOp::Equal => write!(fmt, "=="),
            BinOp::NotEqual => write!(fmt, "!="),
            BinOp::GreaterOrEqual => write!(fmt, ">="),
            BinOp::GreaterThan => write!(fmt, ">"),
            BinOp::LessOrEqual => write!(fmt, "<="),
            BinOp::LessThan => write!(fmt, "<"),
            BinOp::Plus => write!(fmt, "+"),
            BinOp::Minus => write!(fmt, "-"),
            BinOp::Times => write!(fmt, "*"),
            BinOp::Divide => write!(fmt, "/"),
            BinOp::Mod => write!(fmt, "%"),
        }
    }
}


#[derive(Debug)]
pub enum Expr {
    BinExp(Box<Expr>, BinOp, Box<Expr>),
    Not(Box<Expr>),
    Value(Value),
    Var(String),
}

impl fmt::Display for Expr {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // TODO: Print with correct grouping
            Expr::BinExp(ref e1, ref o, ref e2) => write!(fmt, "{} {} {}", e1, o, e2),
            Expr::Not(ref e1) => write!(fmt, "!{}", e1),
            Expr::Value(ref v) => write!(fmt, "{}", v),
            Expr::Var(ref s) => write!(fmt, "{}", s),
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Assign(String, Expr),
    Let(String, Expr),
    Print(Expr),
}

impl fmt::Display for Statement {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Statement::Assign(ref var, ref e) => write!(fmt, "{} = {};", var, e),
            Statement::Let(ref var, ref e) => write!(fmt, "let {} = {};", var, e),
            Statement::Print(ref e) => write!(fmt, "print {};", e),
        }
    }
}
