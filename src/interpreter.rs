use std::collections::HashMap;
use crate::types::BeepboopError;
use crate::types::Expr;
use crate::types::Value;
// use crate::types::Expr::*;
// use crate::types::Value::*;

// pub mod interpreter {
pub fn run_cmd(mut state: ProgramState, cmd: Expr) -> ProgramState {
    match state.eval(cmd) {
        Ok(_val) => state,
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
    pub fn new(exprs: Vec<Expr>) -> Self {
        let state = ProgramState::new();
        Program {exprs, state}
    }
    pub fn run_program(mut self) -> ProgramState {
        self.state = self.exprs.into_iter().fold(self.state, run_cmd);
        self.state
    }
}


// #[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ProgramState {
    pub env: HashMap<String,Value>,
}
impl ProgramState {
    pub fn new() -> Self {
        let env: HashMap<String,Value> = HashMap::new();
        ProgramState {env}
    }
    // pub fn contains(&self, name: String) -> bool {
    //     self.env.contains_key(&name)
    // }
    pub fn assign(&mut self, name: String, val: Value) -> Option<Value> {
        self.env.insert(name,val)
    }
    pub fn lookup(&self, name: String) -> Option<&Value> {
        self.env.get(&name)
    }

    pub fn eval(&mut self, cmd: Expr) -> Result<Value,BeepboopError> {
        match cmd {
            Expr::Assign(name,expr) => {
                let val = self.eval(*expr)?;
                match self.assign(name,val) {
                    Some(_v) => {
                        Ok(val)
                    }
                    None => Ok(val),
                }
            }
            Expr::Lookup(name) => {
                match self.lookup(name) {
                    Some(v) => Ok(*v),
                    None => Err(BeepboopError::SyntaxError),
                }
            }
            Expr::Const(num) => Ok(Value::Num(num)),
            Expr::Plus(expr1,expr2) => {
                let v1 = self.eval(*expr1)?;
                let v2 = self.eval(*expr2)?;
                match (v1,v2) {
                    (Value::Num(n1),Value::Num(n2)) => Ok(Value::Num(n1+n2)),
                    _other => Err(BeepboopError::SyntaxError),
                }
            }
            Expr::Mult(expr1,expr2) => {
                let v1 = self.eval(*expr1)?;
                let v2 = self.eval(*expr2)?;
                match (v1,v2) {
                    (Value::Num(n1),Value::Num(n2)) => Ok(Value::Num(n1*n2)),
                    _other => Err(BeepboopError::SyntaxError),
                }
            }
            Expr::Negate(expr) => {
                let val = self.eval(*expr)?;
                match val {
                    Value::Num(n) => Ok(Value::Num(-n)),
                    Value::Bin(b) => Ok(Value::Bin(!b)),
                    _other => Err(BeepboopError::SyntaxError),
                }

            }
            Expr::And(expr1, expr2) => {
                let v1 = self.eval(*expr1)?;
                let v2 = self.eval(*expr2)?;
                match (v1,v2) {
                    (Value::Bin(b1),Value::Bin(b2)) => Ok(Value::Bin(b1 && b2)),
                    (Value::Num(n1),Value::Num(n2)) => Ok(Value::Bin(n1 != 0 && n2 != 0)),
                    _other => Err(BeepboopError::SyntaxError),
                }
            }
            Expr::Or(expr1, expr2) => {
                let v1 = self.eval(*expr1)?;
                let v2 = self.eval(*expr2)?;
                match (v1,v2) {
                    (Value::Bin(b1),Value::Bin(b2)) => Ok(Value::Bin(b1 || b2)),
                    (Value::Num(n1),Value::Num(n2)) => Ok(Value::Bin(n1 != 0 || n2 != 0)),
                    _other => Err(BeepboopError::SyntaxError),
                }
            }
            Expr::Equal(expr1, expr2) => {
                let v1 = self.eval(*expr1)?;
                let v2 = self.eval(*expr2)?;
                match (v1,v2) {
                    (Value::Num(n1),Value::Num(n2)) => Ok(Value::Bin(n1==n2)),
                    (Value::Bin(b1),Value::Bin(b2)) => Ok(Value::Bin(b1==b2)),
                    _other => Err(BeepboopError::SyntaxError),
                }
            }
            Expr::Greater(expr1, expr2) => {
                let v1 = self.eval(*expr1)?;
                let v2 = self.eval(*expr2)?;
                match (v1,v2) {
                    (Value::Num(n1),Value::Num(n2)) => Ok(Value::Bin(n1 > n2)),
                    _other => Err(BeepboopError::SyntaxError),
                }
            }
            Expr::Less(expr1, expr2) => {
                let v1 = self.eval(*expr1)?;
                let v2 = self.eval(*expr2)?;
                match (v1,v2) {
                    (Value::Num(n1),Value::Num(n2)) => Ok(Value::Bin(n1 < n2)),
                    _other => Err(BeepboopError::SyntaxError),
                }
            }
            Expr::IfThenElse(condition, if_true, if_false) => {
                let vcon = self.eval(*condition)?;
                match vcon {
                    Value::Num(n) => {
                        if n != 0 {
                            self.eval(*if_true)
                        }
                        else {
                            self.eval(*if_false)
                        }
                    }
                    Value::Bin(b) => {
                        if b {
                            self.eval(*if_true)
                        }
                        else {
                            self.eval(*if_false)
                        }
                    }
                    _other => Err(BeepboopError::SyntaxError),
                }
            }
            Expr::For(n, body) => {
                let mut result = self.eval(*body.clone())?;
                let mut vn = self.eval(*n.clone())?;
                let mut loops_done = 1;
                loop {
                    if let Value::Num(n_loops) = vn {
                        println!("loops left: {}",n_loops - loops_done);
                        if n_loops > loops_done {
                            result = self.eval(*body.clone())?;
                            vn = self.eval(*n.clone())?;
                            loops_done += 1;
                        }
                        else {
                            break;
                        }
                    }
                    else {
                        return Err(BeepboopError::SyntaxError);
                    }
                }
                Ok(result)
            }
            // Expr::Function(arg, body, state)
            // Expr::Fold(initial, list, function)
        }
    }
}
// impl Display for ProgramState {
    
// }
