use beepboop::interpreter;
use beepboop::types;
use std::collections::HashMap;

#[test]
fn run_cmd_test_ok() {
    let state = interpreter::ProgramState::new();
    let expr = types::Expr::Assign(String::from("zap"),Box::new(types::Expr::Const(2)));
    let state = interpreter::run_cmd(state,expr);
    let test_env: HashMap<String,beepboop::types::Value> = HashMap::new();
    if let Some(&value) = state.env.get(&String::from("zap")) {
        assert_eq!(value,types::Value::Num(2))
    }
    else {
        assert!(false)
    }
}
