mod test;

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
    Modulus,
}

impl BinOp {
    fn precedence(&self) -> Precedence {
        match *self {
            BinOp::And => Precedence::And,
            BinOp::Or => Precedence::Or,
            BinOp::Equal | BinOp::NotEqual => Precedence::Equality,
            BinOp::GreaterOrEqual | BinOp::GreaterThan | BinOp::LessOrEqual | BinOp::LessThan =>
                Precedence::Inequality,
            BinOp::Plus | BinOp::Minus => Precedence::Addition,
            BinOp::Times | BinOp::Divide | BinOp::Modulus => Precedence::Multiplication,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    // Ordered correctly for derivation to be sound
    Or,
    And,
    Equality,
    Inequality,
    Addition,
    Multiplication,
    Constant,
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
            BinOp::Modulus => write!(fmt, "%"),
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

impl Expr {
    fn precedence(&self) -> Precedence {
        match *self {
            Expr::BinExp(_, ref o, _) => o.precedence(),
            _ => Precedence::Constant,
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expr::BinExp(ref e1, ref o, ref e2) => {
                let po = o.precedence();

                if e1.precedence() < po {
                    try!(write!(fmt, "({})", e1));
                } else {
                    try!(write!(fmt, "{}", e1));
                }

                try!(write!(fmt, " {} ", o));

                if e2.precedence() <= po {
                    write!(fmt, "({})", e2)
                } else {
                    write!(fmt, "{}", e2)
                }
            }
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
