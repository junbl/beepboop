use std::collections::HashMap;
use crate::types::BeepboopError;
use crate::types::Expr;
use crate::types::Value;
use crate::types::Expr::*;
use crate::types::Value::*;

// pub mod interpreter {
fn run_cmd(mut state: ProgramState, cmd: Expr) -> ProgramState {
    match state.eval(cmd) {
        Ok(val) => state,
        Err(error) => {
            eprintln!("{}",error);
            state
        },

    }
}

pub struct Program {
    pub exprs: Vec<Expr>,
    pub state: ProgramState,
}
impl Program {
    pub fn run_program(self) -> ProgramState {
        self.exprs.into_iter().fold(self.state, run_cmd)
    }
}


// #[derive(Copy, Clone)]
pub struct ProgramState {
    pub env: HashMap<String,Value>,
}
impl ProgramState {
    pub fn new() -> Self {
        let env = HashMap::new();
        ProgramState {env}
    }
    pub fn contains(&self, name: String) -> bool {
        self.env.contains_key(&name)
    }
    pub fn assign(&mut self, name: String, val: Value) -> Option<Value> {
        self.env.insert(name,val)
    }
    pub fn lookup(&self, name: String) -> Option<&Value> {
        self.env.get(&name)
    }

    fn eval(&mut self, cmd: Expr) -> Result<Value,BeepboopError> {
        match cmd {
            Assign(name,expr) => {
                let val = self.eval(*expr)?;
                match self.assign(name,val) {
                    Some(v) => {
                        Ok(val)
                    }
                    None => Ok(val),
                }
            }
            Lookup(name) => {
                match self.lookup(name) {
                    Some(v) => Ok(*v),
                    None => Err(BeepboopError::SyntaxError)
                }
            }
            Const(num) => Ok(Num(num)),
            Plus(expr1,expr2) => {
                let v1 = self.eval(*expr1)?;
                let v2 = self.eval(*expr2)?;
                match (v1,v2) {
                    (Num(n1),Num(n2)) => Ok(Num(n1+n2)),
                    other => Err(BeepboopError::SyntaxError),
                }
            }
            Mult(expr1,expr2) => {
                let v1 = self.eval(*expr1)?;
                let v2 = self.eval(*expr2)?;
                match (v1,v2) {
                    (Num(n1),Num(n2)) => Ok(Num(n1*n2)),
                    other => Err(BeepboopError::SyntaxError),
                }
            }
        }
    }
}
// }
