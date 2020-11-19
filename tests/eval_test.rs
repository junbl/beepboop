use beepboop::interpreter;
use beepboop::types;

#[test]
fn eval_test_ok() {
    let mut state = interpreter::ProgramState::new();
    let expr = types::Expr::Const(2);
    assert_eq!(state.eval(expr),Ok(types::Value::Num(2)))
}
