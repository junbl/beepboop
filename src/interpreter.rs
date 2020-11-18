use std::collections::HashMap;
use crate::types::Expr;
use crate::types::Value;
use crate::types::Expr::*;
use crate::types::Value::*;

// pub mod interpreter {
pub struct Program {
    pub exprs: Vec<Expr>,
    pub state: ProgramState,
}
impl Program {
    pub fn run_cmd(&self, state: ProgramState, cmd: Expr) -> Result<ProgramState, &'static str> {
        Ok(state)
    }
    pub fn run_program(&mut self) {
        let initial_state = ProgramState::new();
        self.state = self.exprs.iter().fold(initial_state, |acc,x| -> ProgramState {
            match self.run_cmd(acc,*x) {
                Ok(state) => state,
                Err(error) => {
                    eprintln!("{}",error);
                    acc
                },
            }
        });
    }
}


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
    pub fn lookup(&self, name: String) -> Option<Value> {
        match self.env.get(&name) {
            Some(item) => Some(*item),
            None => None,
        }
    }

    fn eval(&mut self, cmd: Expr) -> Result<Value,&'static str> {
        match cmd {
            Assign(name,expr) => {
                let val = self.eval(*expr)?;
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
                let v1 = self.eval(*expr1)?;
                let v2 = self.eval(*expr2)?;
                match (v1,v2) {
                    (Num(n1),Num(n2)) => Ok(Num(n1+n2)),
                    other => Err("can't add non-number exprs"),
                }
            }
            Mult(expr1,expr2) => {
                let v1 = self.eval(*expr1)?;
                let v2 = self.eval(*expr2)?;
                match (v1,v2) {
                    (Num(n1),Num(n2)) => Ok(Num(n1*n2)),
                    other => Err("can't mult non-number exprs"),
                }
            }
        }
    }
}
// }
