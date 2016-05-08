use ::ast::*;
use ::ast::BinOp::*;
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
    assert_eq!(Value::Int(-36), *state.get("x").unwrap());
    assert_eq!(Value::Int(3), *state.get("y").unwrap());
}
