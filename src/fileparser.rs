use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::types::BeepboopError;
use crate::types::Expr;
use crate::interpreter;

// pub mod fileparser {

pub fn parse_num<'a, I>(num_iter: I) -> Result<i32, BeepboopError>
where
    I: Iterator<Item = &'a str>,
{
    num_iter.fold(Ok(0),|total,digit| -> Result<i32, BeepboopError> {
        match digit {
            "beep" => Ok((total? << 1) + 1),
            "boop" => Ok(total? << 1),
            other => Err(BeepboopError::ParseError), //TODO return iterator. no err handling in here
        }
    })
}

pub fn parse_cmd(cmd: &str) -> Result<Expr, BeepboopError> {
    let mut cmd_iter = cmd.split_whitespace().into_iter();
    match cmd_iter.next() {
        Some("whirr") => {
            if let Some(var_name) = cmd_iter.next() {
                let target_num: i32 = parse_num(cmd_iter)?;
                Ok(Expr::Assign(var_name.to_string(),
                    Box::new(Expr::Const(target_num))))
            }
            else {
                Err(BeepboopError::ParseError)
            }
        },
        // Some("bop") => { // for
        //
        // }
        // Some("bip") => { // if
        //
        // }
        // Some("ding") => { // return
        //
        // }
        // Some("clank") => { // open parenthesis
        //
        // }
        // Some("clonk") => { // close parenthesis
        //
        // }
        None => Err(BeepboopError::SyntaxError),
        _ => Err(BeepboopError::SyntaxError),
    }
}


pub struct BeepboopFile {
    pub lines: Vec<String>,
}

impl BeepboopFile {
    pub fn new(filename: String) -> Self {
        let f = File::open(filename).unwrap();
        let f = BufReader::new(f);

        let lines = f.lines().collect();
        match lines {
            Ok(lines) => BeepboopFile {lines},
            Err(error) => panic!("Couldn't read file! {}",error),
        }
    }
    pub fn parse_file(self) -> interpreter::Program {
        let mut expr_vec: Vec<Expr> = Vec::new();
        for line in self.lines {
            match parse_cmd(&line) {
                Ok(expr) => expr_vec.push(expr),
                Err(error) => panic!("Error while parsing file! {}",error),
            }
        }
        interpreter::Program::new(expr_vec)
    }


    // pub fn run_program() -> Result<ProgramState, Error::ParseError> {
    //     let initial_state = ProgramState::new();
    //     lines.iter().fold(initial_state, run_cmd);
    // }
}
// }
