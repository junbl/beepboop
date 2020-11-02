use std::collections::HashMap;
use crate::types;

pub mod interpreter {
    pub struct Program {
        pub exprs: Vec<Expr>,
        pub state: ProgramState,
    }
    impl Program {
        pub fn run_cmd(state: ProgramState, cmd: Expr) -> Result<ProgramState, Box<dyn Error>> {
            let mut cmd_iter = cmd.split_whitespace();
            match cmd_iter.next() {
                Some("whirr") => {
                    let var_name = cmd_iter.next();
                    let value = cmd_iter.next() match {
                        Some("beep") | Some("boop") => read_num(cmd_iter),
                    }
                },
                None => {},
                other => panic!("Failed to parse at {}",other),
            }
        }
        pub fn run_program() {
            let initial_state = ProgramState::new();
            state = exprs.iter().fold(initial_state, run_cmd);
        }
    }


    pub struct ProgramState {
        pub env: HashMap<String,Expr>,
    }
    impl ProgramState {
        pub fn new() -> Result<ProgramState, &'static str> {
            let mut env = HashMap::new();
            Ok(ProgramState {env})
        }
        pub fn contains(name: String) -> bool {
            env.contains_key(name)
        }
        pub fn assign(name: String, val: Value) -> Option<Value> {
            env.insert(name,val))
        }
        pub fn lookup(name: String) -> Option<Value> {
            env.get(name)
        }

        fn eval(&self, cmd: Expr) -> Result<Value,&'static str> {
            match cmd {
                Assign(name,expr) => {
                    let val = eval(expr);
                    match self.assign(name,val) {
                        Some(v) => Ok(v),
                        None => Ok(val),
                    }
                }
                Lookup(name) => {
                    match self.lookup(name) {
                        Some(v) => Ok(v),
                        None => Err("item not found")
                    }
                }
                Const(num) => Ok(Num(num)),
                Plus(expr1,expr2) => {
                    let v1 = eval(expr1);
                    let v2 = eval(expr2);
                    match v1,v2 {
                        Num(n1),Num(n2) => Ok(Num(n1+n2)),
                        other => Err("can't add non-number exprs"),
                    }
                }
                Mult(expr1,expr2) => {
                    let v1 = eval(expr1);
                    let v2 = eval(expr2);
                    match v1,v2 {
                        Num(n1),Num(n2) => Ok(Num(n1*n2)),
                        other => Err("can't add non-number exprs"),
                    }
                }
            }
        }
    }
}
