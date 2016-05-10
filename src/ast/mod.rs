#[cfg(test)]
mod test;

use std::fmt;

#[derive(Debug)]
pub enum Type {
    Bool,
    Int,
    Str,
    Void,
}

impl fmt::Display for Type {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Bool => write!(fmt, "boolean"),
            Type::Int => write!(fmt, "int"),
            Type::Str => write!(fmt, "string"),
            Type::Void => write!(fmt, "void"),
        }
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Str(String),
}

impl Value {
    pub fn type_string_with_article(&self) -> &str {
        match *self {
            Value::Bool(_) => "a boolean",
            Value::Int(_) => "an int",
            Value::Str(_) => "a string",
        }
    }

    pub fn is_a(&self, t: &Type) -> bool {
        match (self, t) {
            (&Value::Bool(_), &Type::Bool) |
            (&Value::Int(_), &Type::Int) |
            (&Value::Str(_), &Type::Str) => true,
            _ => false
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Bool(b) => write!(fmt, "{}", b),
            Value::Int(i) => write!(fmt, "{}", i),
            Value::Str(ref s) => write!(fmt, "{}", s),
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
    Call(String, Vec<Expr>),
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
            Expr::Call(ref func, ref args) => {
                try!(write!(fmt, "{}(", func));

                for (i, arg) in args.iter().enumerate() {
                    if i != 0 {
                        try!(write!(fmt, ", "));
                    }

                    try!(write!(fmt, "{}", arg));
                }

                write!(fmt, ")")
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
    Defun(Type, String, Vec<String>, Vec<Statement>),
    If(Expr, Vec<Statement>, Vec<Statement>),
    Let(String, Expr),
    Print(Expr),
    While(Expr, Vec<Statement>),
}

impl Statement {
    fn fmt_with_indent(&self, mut fmt: &mut fmt::Formatter, indent_level: u32) -> fmt::Result {
        // New string of `indent_level` spaces.
        let indentation : String = (0..indent_level).map(|_| " ").collect();

        match *self {
            Statement::Assign(ref var, ref e) => writeln!(fmt, "{}{} = {};", indentation, var, e),
            Statement::Defun(ref t, ref name, ref params, ref body) => {
                try!(write!(fmt, "{}{} {}(", indentation, t, name));

                for (i, param) in params.iter().enumerate() {
                    if i != 0 {
                        try!(write!(fmt, ", "));
                    }

                    try!(write!(fmt, "{}", param));
                }

                try!(writeln!(fmt, ") {{"));

                for stmt in body.iter() {
                    try!(stmt.fmt_with_indent(fmt, indent_level + 4));
                }

                writeln!(fmt, "{}}}", indentation)
            }
            Statement::Let(ref var, ref e) => writeln!(fmt, "{}let {} = {};", indentation, var, e),
            Statement::Print(ref e) => writeln!(fmt, "{}print {};", indentation, e),
            Statement::If(ref e, ref v1, ref v2) => {
                try!(writeln!(fmt, "{}if ({}) {{", e, indentation));

                for stmt in v1.iter() {
                    try!(stmt.fmt_with_indent(fmt, indent_level + 4));
                }

                if v2.is_empty() {
                    return Ok(());
                }

                try!(writeln!(fmt, "{}}} else {{", indentation));

                for stmt in v2.iter() {
                    try!(stmt.fmt_with_indent(fmt, indent_level + 4));
                }

                writeln!(fmt, "{}}}", indentation)
            }
            Statement::While(ref e, ref v) => {
                try!(writeln!(fmt, "{}while ({}) {{", e, indentation));

                for stmt in v.iter() {
                    try!(stmt.fmt_with_indent(fmt, indent_level + 4));
                }

                writeln!(fmt, "{}}}", indentation)
            }
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_with_indent(fmt, 0)
    }
}
