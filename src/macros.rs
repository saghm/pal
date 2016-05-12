// Misc -------------------------------------------------------------------------------------------

macro_rules! stringify_from {
    ($id:ident) => (String::from(stringify!($id)))
}

// Expressions ------------------------------------------------------------------------------------
macro_rules! bin_exp {
    ($exp1:expr, $op:expr, $exp2:expr) => (Expr::BinExp(Box::new($exp1), $op, Box::new($exp2)))
}

macro_rules! boolean {
    ($boolean:expr) => (Expr::Value(Value::Bool($boolean)))
}

macro_rules! call {
    ($name:ident ($($arg:expr),*)) => (Expr::Call(stringify_from!($name), vec![$($arg),*]))
}

macro_rules! int {
    ($i:expr) => (Expr::Value(Value::Int($i)))
}

macro_rules! not {
    ($exp:expr) => (Expr::Not(Box::new($exp)))
}

macro_rules! var {
    ($var:ident) => (Expr::Var(stringify_from!($var)))
}

// Statements -------------------------------------------------------------------------------------

macro_rules! stmt_assign {
    ($var:ident, $exp:expr) => (Statement::Assign(stringify_from!($var), $exp))
}

macro_rules! stmt_defun {
    ($ty:expr, $name:ident ($($param:ident),*) { $($stmt:expr);* }) =>
        (Statement::Defun($ty, stringify_from!($name), vec![$(stringify_from!($param)),*], vec![$($stmt),*]))
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

macro_rules! stmt_void_call {
    ($name:ident ($($arg:expr),*)) => (Statement::VoidCall(stringify_from!($name), vec![$($arg),*]))
}

macro_rules! stmt_while {
    ($clause:expr, { $($stmt:expr);* }) => (Statement::While($clause, vec![$($stmt),*]))
}
