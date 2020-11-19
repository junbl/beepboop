use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::types::BeepboopError;
use crate::types::Expr;
use crate::types::Expr::*;
// use crate::types::Value::*;
use crate::interpreter::*;
// use crate::interpreter::ProgramState;

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

pub fn parse_cmd(state: ProgramState, cmd: &str) -> Result<Expr, BeepboopError> {
    let mut cmd_iter = cmd.split_whitespace().into_iter();
    match cmd_iter.next() {
        Some("whirr") => {
            if let Some(var_name) = cmd_iter.next() {
                let target_num: i32 = parse_num(cmd_iter)?;
                Ok(Assign(var_name.to_string(),Box::new(Const(target_num))))
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
        other => Err(BeepboopError::SyntaxError),
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
            Err(error) => panic!("Couldn't read file!"),
        }
    }


    // pub fn run_program() -> Result<ProgramState, Error::ParseError> {
    //     let initial_state = ProgramState::new();
    //     lines.iter().fold(initial_state, run_cmd);
    // }
}
// }
