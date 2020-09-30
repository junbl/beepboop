use std::fs;
use std::collections::HashMap;

pub enum Expr {
    Num(i32),
    Plus(Expr,Expr),
    Mult(Expr,Expr),
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
}


pub struct BeepboopFile {
    pub filename: String,
    pub lines: Vec<&str>,
}

impl BeepboopFile {
    pub fn new(new_filename: &String) -> Result<BeepboopFile, &'static str> {
        let filename = new_filename.clone();
        let contents = fs::read_to_string(filename)?;
        let lines = contents.collect::Vec<&str>();
    }
    fn parse_num(num_iter: Iterator::&str) -> Result<i32,Box<dyn Error>> {
        let mut bin_str = "";
        while let Some(digit) = num_iter.next() {
            match digit {
                "beep" => 
            }
        }
    }
    pub fn run_cmd(state: ProgramState, cmd: &str) -> Result<ProgramState, Box<dyn Error>> {
        let mut cmd_iter = cmd.split_whitespace();
        match cmd_iter.next() {
            Some("whirr") => {
                let var_name = cmd_iter.next();
                let value = cmd_iter.next() match {
                    Some("beep") | Some("boop") => read_num(
                }
            }
            other => eprintln!("Failed to parse at {}",other),
        }
    }
    pub fn run_program() -> Result<ProgramState, Box<dyn Error>> {
        let initial_state = ProgramState::new();
        lines.iter().fold(initial_state, run_cmd);
    }
}
