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

macro_rules! not {
    ($e:expr) => (Expr::Not(Box::new($e)))
}

macro_rules! var {
    ($v:ident) => (Expr::Var(stringify_from!($v)))
}

// Statements -------------------------------------------------------------------------------------

macro_rules! stmt_assign {
    ($v:ident, $e:expr) => (Statement::Assign(stringify_from!($v), $e))
}

macro_rules! stmt_defun {
    ($ty:expr, $name:ident ($($param:ident),*) { $($stmt:expr);* }) =>
        (Statement::Defun($ty, stringify_from!($name), vec![$(stringify_from!($param)),*], vec![$($stmt),*]))
}

macro_rules! stmt_if {
    // if (cond) { ... }
    (($c:expr) { $($stmt:expr);* }) => (Statement::If($c, vec![$($stmt),*], Vec::new()));

    // if (cond) { ... } else { ...}
    (($c:expr) { $($stmt1:expr);* } els { $($stmt2:expr);* }) =>
        (Statement::If($c, vec![$($stmt1),*], vec![$($stmt2),*]));

    // if (cond1) { ... } else if (cond2) { ...}
    (($c1:expr) { $($stmt1:expr);* } elsif ($c2:expr) { $($stmt2:expr);* }) =>
        (Statement::If(($c1), vec![$($stmt1),*], vec![stmt_if!(($c2) { $($stmt2);* } )]));

    // if (cond1) { ... } else if (cond2) { ...} else { ... }
    (($c1:expr) { $($stmt1:expr);* } elsif ($c2:expr) { $($stmt2:expr);* } els { $($stmt3:expr);* }) =>
        (Statement::If(($c1), vec![$($stmt1),*], vec![stmt_if!(($c2) { $($stmt2);* } els { $($stmt3);* })]));

    // if (cond1) { ... } else if (cond2) { ...} else if (cond3) { ... } ... else if (condN) { ...}
    (($c1:expr) { $($stmt1:expr);* } elsif ($c2:expr) { $($stmt2:expr);* } $(elsif ($c3:expr) { $($stmt3:expr);* }),+) =>
        (Statement::If(($c1), vec![$($stmt1),*], vec![
            stmt_if!(($c2) { $($stmt2);* } $(elsif ($c3) { $($stmt3);* }),+)
        ]));

    // if (cond1) { ... } else if (cond2) { ...} else if (cond3) { ... } ... else if (condN) { ...} else { ... }
    (($c1:expr) { $($stmt1:expr);* } elsif ($c2:expr) { $($stmt2:expr);* } $(elsif ($c3:expr) { $($stmt3:expr);* }),+ els { $($stmt4:expr);* }) =>
        (Statement::If(($c1), vec![$($stmt1),*], vec![
            stmt_if!(($c2) { $($stmt2);* } $(elsif ($c3) { $($stmt3);* }),+  els { $($stmt4);* })
        ]))
}

macro_rules! stmt_let {
    ($v:ident, $e:expr) => (Statement::Let(stringify_from!($v), $e))
}

macro_rules! stmt_void_call {
    ($name:ident ($($arg:expr),*)) => (Statement::VoidCall(stringify_from!($name), vec![$($arg),*]))
}

macro_rules! stmt_while {
    ($clause:expr, { $($stmt:expr);* }) => (Statement::While($clause, vec![$($stmt),*]))
}
