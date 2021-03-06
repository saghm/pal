use ast::*;
use ast::BinOp::*;

#[test]
fn display_arith_expr() {
    let exp =
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

    assert_eq!("100 / (-12 / 6) == 4 * 7 % (-6 + 3)", format!("{}", exp));
}

#[test]
fn display_bool_expr() {
    let exp =
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

    assert_eq!("(true || (every_little_thing || false)) != (x && y && (true || is_gonna_be_all_right))", format!("{}", exp));
}

#[test]
fn display_array() {
    let exp = array![int!(-12), boolean!(false), array![var!(y)]];

    assert_eq!("[-12, false, [y]]", format!("{}", exp));
}

#[test]
fn display_for() {
    let stmt = stmt_for!(x <- call!(step(int!(0), var!(y), int!(2))), {
        stmt_print_line!(var!(x))
    });

    let string = "for x in step(0, y, 2) {\n    print_line x;\n}\n";
    assert_eq!(string, format!("{}", stmt));
}

#[test]
fn display_while() {
    let stmt = stmt_while!(bin_exp!(var!(x), GreaterThan, int!(0)), {
        stmt_print_line!(bin_exp!(var!(x), Times, var!(x)))
    });

    let string = "while (x > 0) {\n    print_line x * x;\n}\n";
    assert_eq!(string, format!("{}", stmt));
}


#[test]
fn display_complex_function() {
    let stmt = stmt_defun!(Type::Void, range(i) {
        stmt_while!(bin_exp!(var!(i), GreaterOrEqual, int!(0)), {
            stmt_var_assign!(total, bin_exp!(var!(total), Plus, call!(sum3(var!(i), bin_exp!(var!(i), Plus, int!(1)), bin_exp!(var!(i), Plus, int!(2))))));
            stmt_var_assign!(i, bin_exp!(var!(i), Minus, int!(1)))
        })
    });

    let string = "void range(i) {\n    while (i >= 0) {\n        total = total + sum3(i, i + 1, i + 2);\n        i = i - 1;\n    }\n}\n";
    assert_eq!(string, format!("{}", stmt));
}
