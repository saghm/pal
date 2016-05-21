// Misc -------------------------------------------------------------------------------------------

macro_rules! stringify_from {
    ($id:ident) => (String::from(stringify!($id)))
}

// Values -----------------------------------------------------------------------------------------
macro_rules! val_array {
    ($($val:expr),*) => (Value::Array(vec![$($val),*]))
}


macro_rules! val_string {
    ($string:expr) => (Value::Str(String::from($string)))
}

// Expressions ------------------------------------------------------------------------------------
macro_rules! array {
    ($($exp:expr),*) => (Expr::Array(vec![$($exp),*]))
}

macro_rules! bin_exp {
    ($exp1:expr, $op:expr, $exp2:expr) => (Expr::BinExp(Box::new($exp1), $op, Box::new($exp2)))
}

macro_rules! boolean {
    ($boolean:expr) => (Expr::Value(Value::Bool($boolean)))
}

macro_rules! call {
    ($name:ident ($($arg:expr),*)) => (Expr::Call(stringify_from!($name), vec![$($arg),*]))
}

macro_rules! index {
    ($array:ident[$index1:expr]$([$index2:expr])*) => (Expr::ArrayElement(stringify_from!($array), Box::new($index1), vec![$($index2),*]))
}

macro_rules! int {
    ($i:expr) => (Expr::Value(Value::Int($i)))
}

macro_rules! length {
    ($exp:expr) => (Expr::Length(Box::new($exp)))
}

macro_rules! letters {
    ($exp:expr) => (Expr::Letters(Box::new($exp)))
}


macro_rules! not {
    ($exp:expr) => (Expr::Not(Box::new($exp)))
}

macro_rules! string {
    ($string:expr) => (Expr::Value(val_string!($string)))
}

macro_rules! var {
    ($var:ident) => (Expr::Var(stringify_from!($var)))
}

// Statements -------------------------------------------------------------------------------------

macro_rules! stmt_defun {
    ($ty:expr, $name:ident ($($param:ident),*) { $($stmt:expr);* }) =>
        (Statement::Defun($ty, stringify_from!($name), vec![$(stringify_from!($param)),*], vec![$($stmt),*]))
}

macro_rules! stmt_for {
    ($var:ident <- $arr:expr, { $($stmt:expr);* }) => (Statement::For(stringify_from!($var), $arr, vec![$($stmt),*]))
}

macro_rules! stmt_if {
    // if (cond) { ... }
    (($clause:expr) { $($stmt:expr);* }) => (Statement::If($clause, vec![$($stmt),*], Vec::new()));

    // if (cond) { ... } else { ...}
    (($clause:expr) { $($stmt1:expr);* } els { $($stmt2:expr);* }) =>
        (Statement::If($clause, vec![$($stmt1),*], vec![$($stmt2),*]));

    // if (cond1) { ... } else if (cond2) { ...} else if (cond3) { ... } ... else if (condN) { ...}
    (($clause1:expr) { $($stmt1:expr);* }
    elsif ($clause2:expr) { $($stmt2:expr);* } $(elsif ($clause3:expr) { $($stmt3:expr);* }),*) =>
        (Statement::If(($clause1), vec![$($stmt1),*], vec![
            stmt_if!(($clause2) { $($stmt2);* } $(elsif ($clause3) { $($stmt3);* }),*)
        ]));

    // if (cond1) { ... } else if (cond2) { ...} else if (cond3) { ... } ... else if (condN) { ...} else { ... }
    (($clause1:expr) { $($stmt1:expr);* }
     elsif ($clause2:expr) { $($stmt2:expr);* } $(elsif ($clause3:expr) { $($stmt3:expr);* }),*
     els { $($stmt4:expr);* }) =>
        (Statement::If(($clause1), vec![$($stmt1),*], vec![
            stmt_if!(($clause2) { $($stmt2);* } $(elsif ($clause3) { $($stmt3);* }),*  els { $($stmt4);* })
        ]))
}

macro_rules! stmt_let {
    ($var:ident, $exp:expr) => (Statement::Let(stringify_from!($var), $exp))
}

macro_rules! stmt_print_line {
    ($exp:expr) => (Statement::PrintLine($exp))
}

macro_rules! stmt_var_assign {
    ($var:ident, $exp:expr) => (Statement::VarAssign(stringify_from!($var), $exp))
}

macro_rules! stmt_void_call {
    ($name:ident ($($arg:expr),*)) => (Statement::VoidCall(stringify_from!($name), vec![$($arg),*]))
}

macro_rules! stmt_while {
    ($clause:expr, { $($stmt:expr);* }) => (Statement::While($clause, vec![$($stmt),*]))
}
