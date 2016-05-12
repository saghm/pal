use ::ast::*;
use ::ast::BinOp::*;
use ast::Statement::*;
use ::state::State;

#[test]
fn arith() {
    /*
     * let x = -12;
     * let y = x / -4;
     * x = x * y;
     */

    let stmt1 = stmt_let!(x, int!(-12));
    let stmt2 = stmt_let!(y, bin_exp!(var!(x), Divide, int!(-4)));
    let stmt3 = stmt_assign!(x, bin_exp!(var!(x), Times, var!(y)));

    let mut state = State::new();
    stmt1.eval(&mut state).unwrap();
    stmt2.eval(&mut state).unwrap();
    stmt3.eval(&mut state).unwrap();

    assert_eq!(2, state.len());
    assert_eq!(Value::Int(-36), *state.lookup("x").unwrap());
    assert_eq!(Value::Int(3), *state.lookup("y").unwrap());
}

#[test]
fn bool() {
    /*
     * let x = -12;
     * let y = x > -4;
     * x = true || y;
     */

    let stmt1 = stmt_let!(x, int!(-12));
    let stmt2 = stmt_let!(y, bin_exp!(var!(x), GreaterThan, int!(-4)));
    let stmt3 = stmt_assign!(x, bin_exp!(boolean!(true), Or, var!(y)));

    let mut state = State::new();
    stmt1.eval(&mut state).unwrap();
    stmt2.eval(&mut state).unwrap();
    stmt3.eval(&mut state).unwrap();

    assert_eq!(2, state.len());
    assert_eq!(Value::Bool(true), *state.lookup("x").unwrap());
    assert_eq!(Value::Bool(false), *state.lookup("y").unwrap());
}

#[test]
fn if_true() {
    /*
     * let x = true;
     * let y = -1;
     * if (x || false) {
     *   y = 1;
     * }
     */

     let stmt1 = stmt_let!(x, boolean!(true));
     let stmt2 = stmt_let!(y, int!(-1));
     let stmt3 = stmt_if!((bin_exp!(var!(x), Or, boolean!(false))) {
        stmt_assign!(y, int!(1))
     });

     let mut state = State::new();
     stmt1.eval(&mut state).unwrap();
     stmt2.eval(&mut state).unwrap();
     stmt3.eval(&mut state).unwrap();

     assert_eq!(2, state.len());
     assert_eq!(Value::Bool(true), *state.lookup("x").unwrap());
     assert_eq!(Value::Int(1), *state.lookup("y").unwrap());
}

#[test]
fn if_false() {
    /*
     * let x = true;
     * let y = -1;
     * if (x && false) {
     *   y = 1;
     * }
     */

     let stmt1 = stmt_let!(x, boolean!(true));
     let stmt2 = stmt_let!(y, int!(-1));
     let stmt3 = stmt_if!((bin_exp!(var!(x), And, boolean!(false))) {
        stmt_assign!(y, int!(1))
     });

     let mut state = State::new();
     stmt1.eval(&mut state).unwrap();
     stmt2.eval(&mut state).unwrap();
     stmt3.eval(&mut state).unwrap();

     assert_eq!(2, state.len());
     assert_eq!(Value::Bool(true), *state.lookup("x").unwrap());
     assert_eq!(Value::Int(-1), *state.lookup("y").unwrap());
}

#[test]
fn if_true_else() {
    /*
     * let x = true;
     * let y = -1;
     * if (x || false) {
     *   y = 1;
     * } else {
     *   y = 2;
     * }
     */

    let stmt1 = stmt_let!(x, boolean!(true));
    let stmt2 = stmt_let!(y, int!(-1));
    let stmt3 = stmt_if!((bin_exp!(var!(x), Or, boolean!(false))) {
        stmt_assign!(y, int!(1))
    } els {
        stmt_assign!(y, int!(2))
    });

     let mut state = State::new();
     stmt1.eval(&mut state).unwrap();
     stmt2.eval(&mut state).unwrap();
     stmt3.eval(&mut state).unwrap();

     assert_eq!(2, state.len());
     assert_eq!(Value::Bool(true), *state.lookup("x").unwrap());
     assert_eq!(Value::Int(1), *state.lookup("y").unwrap());
}

#[test]
fn if_false_else() {
    /*
     * let x = true;
     * let y = -1;
     * if (x || false) {
     *   y = 1;
     * } else {
     *   y = 2;
     * }
     */

    let stmt1 = stmt_let!(x, boolean!(true));
    let stmt2 = stmt_let!(y, int!(-1));
    let stmt3 = stmt_if!((bin_exp!(var!(x), And, boolean!(false))) {
        stmt_assign!(y, int!(1))
    } els {
        stmt_assign!(y, int!(2))
    });

     let mut state = State::new();
     stmt1.eval(&mut state).unwrap();
     stmt2.eval(&mut state).unwrap();
     stmt3.eval(&mut state).unwrap();

     assert_eq!(2, state.len());
     assert_eq!(Value::Bool(true), *state.lookup("x").unwrap());
     assert_eq!(Value::Int(2), *state.lookup("y").unwrap());
}

#[test]
fn if_true_else_if2() {
    /*
     * let x = true;
     * let y = -1;
     * if (x || false) {
     *   y = 1;
     * } else if (x && false) {
     *   y = 2;
     * } else if (x) {
     *   y = 3;
     * }
     */

    let stmt1 = stmt_let!(x, boolean!(true));
    let stmt2 = stmt_let!(y, int!(-1));
    let stmt3 = stmt_if!((bin_exp!(var!(x), Or, boolean!(false))) {
        stmt_assign!(y, int!(1))
    } elsif (bin_exp!(var!(x), And, boolean!(false))) {
        stmt_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_assign!(y, int!(3))
    });

    let mut state = State::new();
    stmt1.eval(&mut state).unwrap();
    stmt2.eval(&mut state).unwrap();
    stmt3.eval(&mut state).unwrap();

    assert_eq!(2, state.len());
    assert_eq!(Value::Bool(true), *state.lookup("x").unwrap());
    assert_eq!(Value::Int(1), *state.lookup("y").unwrap());
}

#[test]
fn if_false_else_if_true_else_if() {
    /*
     * let x = true;
     * let y = -1;
     * if (x && false) {
     *   y = 1;
     * } else if (x || false) {
     *   y = 2;
     * } else if (x) {
     *   y = 3;
     * }
     */

    let stmt1 = stmt_let!(x, boolean!(true));
    let stmt2 = stmt_let!(y, int!(-1));
    let stmt3 = stmt_if!((bin_exp!(var!(x), And, boolean!(false))) {
        stmt_assign!(y, int!(1))
    } elsif (bin_exp!(var!(x), Or, boolean!(false))) {
        stmt_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_assign!(y, int!(3))
    });

    let mut state = State::new();
    stmt1.eval(&mut state).unwrap();
    stmt2.eval(&mut state).unwrap();
    stmt3.eval(&mut state).unwrap();

    assert_eq!(2, state.len());
    assert_eq!(Value::Bool(true), *state.lookup("x").unwrap());
    assert_eq!(Value::Int(2), *state.lookup("y").unwrap());
}

#[test]
fn if_false_else_if_false_else_if_true() {
    /*
     * let x = true;
     * let y = -1;
     * if (x && false) {
     *   y = 1;
     * } else if (!x || false) {
     *   y = 2;
     * } else if (x) {
     *   y = 3;
     * }
     */


    let stmt1 = stmt_let!(x, boolean!(true));
    let stmt2 = stmt_let!(y, int!(-1));
    let stmt3 = stmt_if!((bin_exp!(var!(x), And, boolean!(false))) {
        stmt_assign!(y, int!(1))
    } elsif (bin_exp!(not!(var!(x)), Or, boolean!(false))) {
        stmt_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_assign!(y, int!(3))
    });

    let mut state = State::new();
    stmt1.eval(&mut state).unwrap();
    stmt2.eval(&mut state).unwrap();
    stmt3.eval(&mut state).unwrap();

    assert_eq!(2, state.len());
    assert_eq!(Value::Bool(true), *state.lookup("x").unwrap());
    assert_eq!(Value::Int(3), *state.lookup("y").unwrap());
}

#[test]
fn if_true_else_if2_else() {
    /*
     * let x = true;
     * let y = -1;
     * if (x || false) {
     *   y = 1;
     * } else if (x && false) {
     *   y = 2;
     * } else if (x) {
     *   y = 3;
     * } else {
     *   y = 4;
     * }
     */

    let stmt1 = stmt_let!(x, boolean!(true));
    let stmt2 = stmt_let!(y, int!(-1));
    let stmt3 = stmt_if!((bin_exp!(var!(x), Or, boolean!(false))) {
        stmt_assign!(y, int!(1))
    } elsif (bin_exp!(var!(x), And, boolean!(false))) {
        stmt_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_assign!(y, int!(3))
    } els {
        stmt_assign!(y, int!(4))
    });

    let mut state = State::new();
    stmt1.eval(&mut state).unwrap();
    stmt2.eval(&mut state).unwrap();
    stmt3.eval(&mut state).unwrap();

    assert_eq!(2, state.len());
    assert_eq!(Value::Bool(true), *state.lookup("x").unwrap());
    assert_eq!(Value::Int(1), *state.lookup("y").unwrap());
}

#[test]
fn if_false_else_if2_true_else() {
    /*
     * let x = true;
     * let y = -1;
     * if (x && false) {
     *   y = 1;
     * } else if (x || false) {
     *   y = 2;
     * } else if (x) {
     *   y = 3;
     * } else {
     *   y = 4;
     * }
     */

    let stmt1 = stmt_let!(x, boolean!(true));
    let stmt2 = stmt_let!(y, int!(-1));
    let stmt3 = stmt_if!((bin_exp!(var!(x), And, boolean!(false))) {
        stmt_assign!(y, int!(1))
    } elsif (bin_exp!(var!(x), Or, boolean!(false))) {
        stmt_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_assign!(y, int!(3))
    } els {
        stmt_assign!(y, int!(4))
    });

    let mut state = State::new();
    stmt1.eval(&mut state).unwrap();
    stmt2.eval(&mut state).unwrap();
    stmt3.eval(&mut state).unwrap();

    assert_eq!(2, state.len());
    assert_eq!(Value::Bool(true), *state.lookup("x").unwrap());
    assert_eq!(Value::Int(2), *state.lookup("y").unwrap());
}

#[test]
fn if_false_else_if2_false_else_true() {
    /*
     * let x = false;
     * let y = -1;
     * if (x || false) {
     *   y = 1;
     * } else if (x && false) {
     *   y = 2;
     * } else if (x) {
     *   y = 3;
     * } else {
     *   y = 4;
     * }
     */

    let stmt1 = stmt_let!(x, boolean!(false));
    let stmt2 = stmt_let!(y, int!(-1));
    let stmt3 = stmt_if!((bin_exp!(var!(x), And, boolean!(false))) {
        stmt_assign!(y, int!(1))
    } elsif (bin_exp!(var!(x), Or, boolean!(false))) {
        stmt_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_assign!(y, int!(3))
    } els {
        stmt_assign!(y, int!(4))
    });

    let mut state = State::new();
    stmt1.eval(&mut state).unwrap();
    stmt2.eval(&mut state).unwrap();
    stmt3.eval(&mut state).unwrap();

    assert_eq!(2, state.len());
    assert_eq!(Value::Bool(false), *state.lookup("x").unwrap());
    assert_eq!(Value::Int(4), *state.lookup("y").unwrap());
}

#[test]
fn complex() {
    /*
     * let total = 0;
     *
     * int sum3(x, y, z) {
     *   return x + y + z;
     * }
     *
     * void range(i) {
     *   while (i >= 0) {
     *     total = total + sum3(i, i + 1, i + 2);
     *     i = i - 1;
     *   }
     * }
     *
     * range(10);
     */

    let let_total = stmt_let!(total, int!(0));

    let sum3 = stmt_defun!(Type::Int, sum3(x, y, z) {
        Return(bin_exp!(var!(x), Plus, bin_exp!(var!(y), Plus, var!(z))))
    });

    let range = stmt_defun!(Type::Void, range(i) {
        stmt_while!(bin_exp!(var!(i), GreaterOrEqual, int!(0)), {
            stmt_assign!(total, bin_exp!(var!(total), Plus, call!(sum3(var!(i), bin_exp!(var!(i), Plus, int!(1)), bin_exp!(var!(i), Plus, int!(2))))));
            stmt_assign!(i, bin_exp!(var!(i), Minus, int!(1)))
        })
    });

    let range_call = stmt_void_call!(range(int!(10)));

    let mut state = State::new();
    let_total.eval(&mut state).unwrap();
    sum3.eval(&mut state).unwrap();
    range.eval(&mut state).unwrap();
    range_call.eval(&mut state).unwrap();

    assert_eq!(1, state.len());
    assert_eq!(Value::Int(198), *state.lookup("total").unwrap());
}
