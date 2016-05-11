// Misc -------------------------------------------------------------------------------------------

macro_rules! stringify_from {
    ($id:ident) => (String::from(stringify!($id)))
}

// Expressions ------------------------------------------------------------------------------------
macro_rules! bin_exp {
    ($e1:expr, $o:expr, $e2:expr) => (Expr::BinExp(Box::new($e1), $o, Box::new($e2)))
}

macro_rules! boolean {
    ($b:expr) => (Expr::Value(Value::Bool($b)))
}

macro_rules! call {
    ($name:ident ($($arg:expr),*)) => (Expr::Call(stringify_from!($name), vec![$($arg),*]))
}

macro_rules! int {
    ($i:expr) => (Expr::Value(Value::Int($i)))
}

macro_rules! var {
    ($v:ident) => (Expr::Var(stringify_from!($v)))
}

// Statements -------------------------------------------------------------------------------------
macro_rules! stmt_let {
    ($v:ident, $e:expr) => (Statement::Let(stringify_from!($v), $e))
}

macro_rules! stmt_assign {
    ($v:ident, $e:expr) => (Statement::Assign(stringify_from!($v), $e))
}

macro_rules! stmt_while {
    ($clause:expr, { $($stmt:expr);* }) => (Statement::While($clause, vec![$($stmt),*]))
}

macro_rules! defun {
    ($ty:expr, $name:ident ($($param:ident),*) { $($stmt:expr);* }) =>
        (Statement::Defun($ty, stringify_from!($name), vec![$(stringify_from!($param)),*], vec![$($stmt),*]))
}

macro_rules! void_call {
    ($name:ident ($($arg:expr),*)) => (Statement::VoidCall(stringify_from!($name), vec![$($arg),*]))
}
