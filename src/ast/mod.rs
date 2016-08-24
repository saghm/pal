#[cfg(test)]
mod test;

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Array,
    Bool,
    Int,
    Str,
    Void,
}

impl Type {
    pub fn as_string_with_article(&self) -> &str {
        // Returns the name of the type with the correct English indefinite article prepended.
        match *self {
            Type::Array => "an array",
            Type::Bool => "a boolean",
            Type::Int => "an int",
            Type::Str => "a string",
            Type::Void => "nothing",
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, mut fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Type::Array => write!(fmt, "array"),
            Type::Bool => write!(fmt, "boolean"),
            Type::Int => write!(fmt, "int"),
            Type::Str => write!(fmt, "string"),
            Type::Void => write!(fmt, "void"),
        }
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Array(Vec<Value>),
    Bool(bool),
    Int(i64),
    Str(String),
}

impl Value {
    // Returns the value's type with the correct English indefinite article prepended.
    pub fn type_string_with_article(&self) -> &str {
        match *self {
            Value::Array(_) => "an array",
            Value::Bool(_) => "a boolean",
            Value::Int(_) => "an int",
            Value::Str(_) => "a string",
        }
    }

    // Checks whether the value is of a certain type.
    pub fn is_a(&self, t: &Type) -> bool {
        match (self, t) {
            (&Value::Array(_), &Type::Array) |
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
            Value::Array(ref vec) => {
                try!(write!(fmt, "["));

                for (i, val) in vec.iter().enumerate() {
                    if i != 0 {
                        try!(write!(fmt, ", "));
                    }

                    try!(write!(fmt, "{}", val));
                }

                write!(fmt, "]")
            }
            Value::Bool(b) => write!(fmt, "{}", b),
            Value::Int(i) => write!(fmt, "{}", i),
            Value::Str(ref s) => write!(fmt, "{}", s),
        }
    }
}

#[derive(Clone, Debug)]
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

    // Other
    Concat,
}

impl BinOp {
    // Gets the precedence of an operator.
    fn precedence(&self) -> Precedence {
        match *self {
            BinOp::And => Precedence::And,
            BinOp::Or => Precedence::Or,
            BinOp::Equal | BinOp::NotEqual => Precedence::Equality,
            BinOp::GreaterOrEqual | BinOp::GreaterThan | BinOp::LessOrEqual | BinOp::LessThan =>
                Precedence::Inequality,
            BinOp::Plus | BinOp::Minus | BinOp::Concat => Precedence::Addition,
            BinOp::Times | BinOp::Divide | BinOp::Modulus => Precedence::Multiplication,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    // Ordered correctly for derivation to be sound; each variant has higher precedence than the previous
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
            BinOp::Concat => write!(fmt, "++"),
        }
    }
}


#[derive(Clone, Debug)]
pub enum Expr {
    Array(Vec<Expr>),
    ArrayElement(String, Box<Expr>, Vec<Expr>),
    BinExp(Box<Expr>, BinOp, Box<Expr>),
    Call(String, Vec<Expr>),
    Length(Box<Expr>),
    Letters(Box<Expr>),
    Not(Box<Expr>),
    Range(Box<Expr>, Box<Expr>),
    ReadLine,
    Step(Box<Expr>, Box<Expr>, Box<Expr>),
    Value(Value),
    Var(String),
}

impl Expr {
    // Returns the precedence level of the expression.
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
            Expr::Array(ref vec) => {
                try!(write!(fmt, "["));

                for (i, val) in vec.iter().enumerate() {
                    if i != 0 {
                        try!(write!(fmt, ", "));
                    }

                    try!(write!(fmt, "{}", val));
                }

                write!(fmt, "]")
            }
            Expr::ArrayElement(ref var, ref index, ref indexes) => {
                try!(write!(fmt, "{}[{}]", var, index));

                for i in indexes.iter() {
                    try!(write!(fmt, "[{}]", i));
                }

                Ok(())
            }
            Expr::BinExp(ref exp1, ref op, ref exp2) => {
                let op_precendence = op.precedence();

                // Wrap the left-hand side in parentheses if its precedence is lower than the operator
                if exp1.precedence() < op_precendence {
                    try!(write!(fmt, "({})", exp1));
                } else {
                    try!(write!(fmt, "{}", exp1));
                }

                try!(write!(fmt, " {} ", op));

                // Wrap the left-hand side in parentheses if its precedence is not greater than the operator
                if exp2.precedence() <= op_precendence {
                    write!(fmt, "({})", exp2)
                } else {
                    write!(fmt, "{}", exp2)
                }
            }
            Expr::Call(ref func, ref args) => {
                try!(write!(fmt, "{}(", func));

                // Write the arguments, separated by commas
                for (i, arg) in args.iter().enumerate() {
                    if i != 0 {
                        try!(write!(fmt, ", "));
                    }

                    try!(write!(fmt, "{}", arg));
                }

                write!(fmt, ")")
            }
            Expr::Length(ref exp) => write!(fmt, "length({})", exp),
            Expr::Letters(ref exp) => write!(fmt, "letters({})", exp),
            Expr::Not(ref exp) => write!(fmt, "!{}", exp),
            Expr::Range(ref start, ref end) => write!(fmt, "range({}, {})", start, end),
            Expr::ReadLine => write!(fmt, "readline()"),
            Expr::Step(ref start, ref end, ref step) => write!(fmt, "step({}, {}, {})", start, end, step),
            Expr::Value(ref val) => write!(fmt, "{}", val),
            Expr::Var(ref var) => write!(fmt, "{}", var),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    ArrayElemAssign(String, Expr, Vec<Expr>, Expr),
    For(String, Expr, Vec<Statement>),
    Defun(Type, String, Vec<String>, Vec<Statement>),
    Delete(String, Expr, Vec<Expr>),
    If(Expr, Vec<Statement>, Vec<Statement>),
    Let(String, Expr),
    Print(Expr),
    PrintLine(Expr),
    Return(Expr),
    VoidCall(String, Vec<Expr>),
    While(Expr, Vec<Statement>),
    VarAssign(String, Expr),
}

impl Statement {
    // Formats the expression with indentation before it.
    fn fmt_with_indent(&self, mut fmt: &mut fmt::Formatter, indent_level: u32) -> fmt::Result {
        // Creates a new string that is as many spaces as `indent level * 4`.
        let indentation : String = (0..indent_level * 4).map(|_| " ").collect();

        match *self {
            Statement::ArrayElemAssign(ref var, ref index, ref indexes, ref exp) => {
                try!(write!(fmt, "{}{}[{}]", indentation, var, index));

                for i in indexes.iter() {
                    try!(write!(fmt, "[{}]", i));
                }

                writeln!(fmt, " = {};", exp)
            }
            Statement::Defun(ref return_type, ref name, ref params, ref body) => {
                try!(write!(fmt, "{}{} {}(", indentation, return_type, name));

                // Write the parameters, separated by commas
                for (i, param) in params.iter().enumerate() {
                    if i != 0 {
                        try!(write!(fmt, ", "));
                    }

                    try!(write!(fmt, "{}", param));
                }

                try!(writeln!(fmt, ") {{"));

                // Write the function body statements with one more level of indentation
                for stmt in body.iter() {
                    try!(stmt.fmt_with_indent(fmt, indent_level + 1));
                }

                writeln!(fmt, "{}}}", indentation)
            }
            Statement::Delete(ref var, ref index, ref indexes) => {
                try!(write!(fmt, "{}delete {}[{}]", indentation, var, index));

                for i in indexes.iter() {
                    try!(write!(fmt, "[{}]", i));
                }

                Ok(())
            }
            Statement::For(ref var, ref exp, ref block) => {
                try!(writeln!(fmt, "{}for {} in {} {{", indentation, var, exp));

                for stmt in block.iter() {
                    try!(stmt.fmt_with_indent(fmt, indent_level + 1));
                }

                writeln!(fmt, "{}}}", indentation)
            }
            Statement::If(ref clause, ref true_block, ref false_block) => {
                try!(writeln!(fmt, "{}if ({}) {{", indentation, clause));

                // Write the block statements with one more level of indentation
                for stmt in true_block.iter() {
                    try!(stmt.fmt_with_indent(fmt, indent_level + 1));
                }

                // Don't write the "else" clause/block unless there is something in the block.
                if false_block.is_empty() {
                    return Ok(());
                }

                try!(writeln!(fmt, "{}}} else {{", indentation));

                // Write the block statements with one more level of indentation
                for stmt in false_block.iter() {
                    try!(stmt.fmt_with_indent(fmt, indent_level + 1));
                }

                writeln!(fmt, "{}}}", indentation)
            }
            Statement::Let(ref var, ref exp) => writeln!(fmt, "{}let {} = {};", indentation, var, exp),
            Statement::Print(ref exp) => writeln!(fmt, "{}print {};", indentation, exp),
            Statement::PrintLine(ref exp) => writeln!(fmt, "{}print_line {};", indentation, exp),
            Statement::Return(ref exp) => writeln!(fmt, "{}return {};", indentation, exp),
            Statement::VarAssign(ref var, ref exp) => writeln!(fmt, "{}{} = {};", indentation, var, exp),
            Statement::VoidCall(ref name, ref args) => {
                try!(write!(fmt, "{}{}(", indentation, name));

                // Write the arguments, separated by commas
                for (i, arg) in args.iter().enumerate() {
                    if i != 0 {
                        try!(write!(fmt, ", "));
                    }

                    try!(write!(fmt, "{}", arg));
                }

                write!(fmt, ");")
            }
            Statement::While(ref clause, ref block) => {
                try!(writeln!(fmt, "{}while ({}) {{", indentation, clause));

                // Write the block statements with one more level of indentation
                for stmt in block.iter() {
                    try!(stmt.fmt_with_indent(fmt, indent_level + 1));
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
