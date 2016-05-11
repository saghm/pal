use ::ast::*;
use ::ast::BinOp::*;
use ast::Statement::*;
use ::state::State;

#[test]
fn arith() {
    // let x = -12;
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
    // let x = -12;
    let stmt1 = stmt_let!(x, int!(-12));
    let stmt2 = stmt_let!(y, bin_exp!(var!(x), GreaterOrEqual, int!(-4)));
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
fn functions() {
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

    let sum3 = defun!(Type::Int, sum3(x, y, z) {
        Return(bin_exp!(var!(x), Plus, bin_exp!(var!(y), Plus, var!(z))))
    });

    let range = defun!(Type::Void, range(i) {
        stmt_while!(bin_exp!(var!(i), GreaterOrEqual, int!(0)), {
            stmt_assign!(total, bin_exp!(var!(total), Plus, call!(sum3(var!(i), bin_exp!(var!(i), Plus, int!(1)), bin_exp!(var!(i), Plus, int!(2))))));
            stmt_assign!(i, bin_exp!(var!(i), Minus, int!(1)))
        })
    });

    let range_call = void_call!(range(int!(10)));

    let mut state = State::new();
    let_total.eval(&mut state).unwrap();
    sum3.eval(&mut state).unwrap();
    range.eval(&mut state).unwrap();
    range_call.eval(&mut state).unwrap();

    assert_eq!(1, state.len());
    assert_eq!(Value::Int(198), *state.lookup("total").unwrap());
}
