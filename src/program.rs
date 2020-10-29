use std::collections::HashMap;

pub mod program {
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
                other => eprintln!("Failed to parse at {}",other),
            }
        }
        pub fn run_program() -> Result<ProgramState, Box<dyn Error>> {
            let initial_state = ProgramState::new();
            lines.iter().fold(initial_state, run_cmd);
    }


    pub struct ProgramState {
        pub env: HashMap<String,Expr>,
    }
    impl ProgramState {
        pub fn new() -> Result<ProgramState, &'static str> {
            let mut env = HashMap::new();
            Ok(ProgramState {env})
        }
        pub fn assign(name: String, val: Expr) {
            env.insert(name, val);
        }
        pub fn lookup(name: String) -> Option {
            env.get(name)
        }
    }
}
