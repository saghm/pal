use ast::*;
use ast::BinOp::*;
use ast::Statement::*;
use error::ErrorType;
use state::State;

#[test]
fn arith() {
    /*
     * let x = -12;
     * let y = x / -4;
     * x = x * y;
     */

    let stmt1 = stmt_let!(x, int!(-12));
    let stmt2 = stmt_let!(y, bin_exp!(var!(x), Divide, int!(-4)));
    let stmt3 = stmt_var_assign!(x, bin_exp!(var!(x), Times, var!(y)));

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
    let stmt3 = stmt_var_assign!(x, bin_exp!(boolean!(true), Or, var!(y)));

    let mut state = State::new();
    stmt1.eval(&mut state).unwrap();
    stmt2.eval(&mut state).unwrap();
    stmt3.eval(&mut state).unwrap();

    assert_eq!(2, state.len());
    assert_eq!(Value::Bool(true), *state.lookup("x").unwrap());
    assert_eq!(Value::Bool(false), *state.lookup("y").unwrap());
}

#[test]
fn array_element_access() {
    // let array = [1, false, ["hello"]];
    let stmt = stmt_let!(array, array![int!(1), boolean!(false), array![string!("hello!")]]);

    let mut state = State::new();
    stmt.eval(&mut state).unwrap();

    let index_neg_one = index!(array[int!(-1)]);
    assert_eq!(Err(ErrorType::ArrayIndexOutOfBounds), index_neg_one.eval(&mut state).map_err(|e| e.err_type()));

    let index_zero = index!(array[int!(0)]);
    assert_eq!(Value::Int(1), index_zero.eval(&mut state).unwrap());

    let index_one = index!(array[int!(1)]);
    assert_eq!(Value::Bool(false), index_one.eval(&mut state).unwrap());

    let index_one_zero = index!(array[int!(1)][int!(0)]);
    assert_eq!(Err(ErrorType::Type), index_one_zero.eval(&mut state).map_err(|e| e.err_type()));

    let index_two_neg_one = index!(array[int!(2)][int!(-1)]);
    assert_eq!(Err(ErrorType::ArrayIndexOutOfBounds), index_two_neg_one.eval(&mut state).map_err(|e| e.err_type()));

    let index_two_zero = index!(array[int!(2)][int!(0)]);
    assert_eq!(Value::Str(String::from("hello!")), index_two_zero.eval(&mut state).unwrap());

    let index_two_one = index!(array[int!(2)][int!(1)]);
    assert_eq!(Err(ErrorType::ArrayIndexOutOfBounds), index_two_one.eval(&mut state).map_err(|e| e.err_type()));

    let index_three = index!(array[int!(3)]);
    assert_eq!(Err(ErrorType::ArrayIndexOutOfBounds), index_three.eval(&mut state).map_err(|e| e.err_type()));
}

#[test]
fn length_array() {
    let array1 = array![int!(1), boolean!(false), array![string!("hello!"), int!(0)]];
    let array2 = array![];

    let length1 = length!(array1);
    let length2 = length!(array2);

    let mut state = State::new();

    assert_eq!(Value::Int(3), length1.eval(&mut state).unwrap());
    assert_eq!(Value::Int(0), length2.eval(&mut state).unwrap());
}

#[test]
fn length_string() {
    let length = length!(string!("hello!"));

    let mut state = State::new();

    assert_eq!(Value::Int(6), length.eval(&mut state).unwrap());
}

#[test]
fn length_invalid_arg() {
    let bool_length = length!(boolean!(false));
    let int_length = length!(bin_exp!(int!(10), Divide, int!(3)));

    let mut state = State::new();

    assert_eq!(Err(ErrorType::Type), bool_length.eval(&mut state).map_err(|e| e.err_type()));
    assert_eq!(Err(ErrorType::Type), int_length.eval(&mut state).map_err(|e| e.err_type()));
}

#[test]
fn letters_string() {
    let length1 = letters!(string!(""));
    let length2 = letters!(string!("hello!"));

    let array1 = val_array![];
    let array2 = val_array![ val_string!("h")
                           , val_string!("e")
                           , val_string!("l")
                           , val_string!("l")
                           , val_string!("o")
                           , val_string!("!")
                           ];

    let mut state = State::new();

    assert_eq!(array1, length1.eval(&mut state).unwrap());
    assert_eq!(array2, length2.eval(&mut state).unwrap());
}

#[test]
fn letters_invalid_arg() {
    let array_letters = letters!(array![string!("hello!")]);
    let bool_letters = letters!(boolean!(false));
    let int_letters = letters!(bin_exp!(int!(10), Divide, int!(3)));

    let mut state = State::new();

    assert_eq!(Err(ErrorType::Type), array_letters.eval(&mut state).map_err(|e| e.err_type()));
    assert_eq!(Err(ErrorType::Type), bool_letters.eval(&mut state).map_err(|e| e.err_type()));
    assert_eq!(Err(ErrorType::Type), int_letters.eval(&mut state).map_err(|e| e.err_type()));
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
        stmt_var_assign!(y, int!(1))
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
        stmt_var_assign!(y, int!(1))
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
        stmt_var_assign!(y, int!(1))
    } els {
        stmt_var_assign!(y, int!(2))
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
        stmt_var_assign!(y, int!(1))
    } els {
        stmt_var_assign!(y, int!(2))
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
        stmt_var_assign!(y, int!(1))
    } elsif (bin_exp!(var!(x), And, boolean!(false))) {
        stmt_var_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_var_assign!(y, int!(3))
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
        stmt_var_assign!(y, int!(1))
    } elsif (bin_exp!(var!(x), Or, boolean!(false))) {
        stmt_var_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_var_assign!(y, int!(3))
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
        stmt_var_assign!(y, int!(1))
    } elsif (bin_exp!(not!(var!(x)), Or, boolean!(false))) {
        stmt_var_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_var_assign!(y, int!(3))
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
        stmt_var_assign!(y, int!(1))
    } elsif (bin_exp!(var!(x), And, boolean!(false))) {
        stmt_var_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_var_assign!(y, int!(3))
    } els {
        stmt_var_assign!(y, int!(4))
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
        stmt_var_assign!(y, int!(1))
    } elsif (bin_exp!(var!(x), Or, boolean!(false))) {
        stmt_var_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_var_assign!(y, int!(3))
    } els {
        stmt_var_assign!(y, int!(4))
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
        stmt_var_assign!(y, int!(1))
    } elsif (bin_exp!(var!(x), Or, boolean!(false))) {
        stmt_var_assign!(y, int!(2))
    } elsif (var!(x)) {
        stmt_var_assign!(y, int!(3))
    } els {
        stmt_var_assign!(y, int!(4))
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
            stmt_var_assign!(total, bin_exp!(var!(total), Plus, call!(sum3(var!(i), bin_exp!(var!(i), Plus, int!(1)), bin_exp!(var!(i), Plus, int!(2))))));
            stmt_var_assign!(i, bin_exp!(var!(i), Minus, int!(1)))
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
