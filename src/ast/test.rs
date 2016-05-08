#[cfg(test)]
mod tests {
    use ::ast::*;
    use ::ast::BinOp::*;

    macro_rules! bin_exp {
        ($e1:expr, $o:expr, $e2:expr) => (Expr::BinExp(Box::new($e1), $o, Box::new($e2)))
    }

    macro_rules! int {
        ($i:expr) => (Expr::Value(Value::Int($i)))
    }

    macro_rules! boolean {
        ($b:expr) => (Expr::Value(Value::Bool($b)))
    }

    macro_rules! var {
        ($v:ident) => (Expr::Var(String::from(stringify!($v))))
    }

    #[test]
    fn display_arith_expr() {
        let e =
            bin_exp!(
                bin_exp!(
                    int!(100),
                    Divide,
                    bin_exp!(int!(-12), Divide, int!(6))),
                Equal,
                bin_exp!(
                    bin_exp!(int!(4), Times, int!(7)),
                    Modulus,
                    bin_exp!(int!(-6), Plus, int!(3))));

        assert_eq!("100 / (-12 / 6) == 4 * 7 % (-6 + 3)", format!("{}", e));
    }

    #[test]
    fn display_bool_expr() {
        let e =
            bin_exp!(
                bin_exp!(
                    boolean!(true),
                    Or,
                    bin_exp!(var!(every_little_thing), Or, boolean!(false))),
                NotEqual,
                bin_exp!(
                    bin_exp!(var!(x), And, var!(y)),
                    And,
                    bin_exp!(boolean!(true), Or, var!(is_gonna_be_all_right))));

        assert_eq!("(true || (every_little_thing || false)) != (x && y && (true || is_gonna_be_all_right))", format!("{}", e));
    }
}
