use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::types::BeepboopError;
use crate::types::Expr;
use crate::interpreter;

// pub mod fileparser {

pub fn parse_num<'a, I>(num_iter: &mut I) -> Result<i32, BeepboopError>
where
    I: Iterator<Item = &'a str>,
{
    let mut total = 0;
    while let Some(digit) = num_iter.next() {
        match digit {
            "beep" => total = (total << 1) + 1,
            "boop" => total = total << 1,
            "clonk" => break,
            other => break,
        }
    }
    Ok(total)
}

// pub fn parse_if_then_else<'a, I>(iter: &mut I) -> Result<Expr, BeepboopError>
// where
//     I: Iterator<Item = &'a str>,
// {


// }

pub fn parse_cmd(cmd: &str) -> Result<Expr, BeepboopError> {
    let mut cmd_iter = cmd.split_whitespace().into_iter();
    parse_cmd_helper(&mut cmd_iter)
}

pub fn parse_cmd_helper<'a, I>(cmd_iter: &mut I) -> Result<Expr, BeepboopError>
where
    I: Iterator<Item = &'a str>,
{
    match cmd_iter.next() {
        Some("boop") => { // numbers
            let target_num: i32 = parse_num(cmd_iter)?;
            Ok(Expr::Const(target_num))
        }
        // Some("beep") => { // numbers
        //     let target_num: i32 = (parse_num(&mut cmd_iter)? << 1) + 1;
        //     Ok(Expr::Const(target_num))
        // }
        Some("whirr") => { // variable assignment
            if let Some(var_name) = cmd_iter.next() {
                let target_expr = parse_cmd_helper(cmd_iter)?;
                Ok(Expr::Assign(var_name.to_string(),
                    Box::new(target_expr)))
            }
            else {
                Err(BeepboopError::ParseError)
            }
        },
        Some("plop") => { // plus
            // println!("addition");
            if cmd_iter.next() == Some("clank") {
                let e1 = parse_cmd_helper(cmd_iter)?;
                if cmd_iter.next() == Some("clank") {
                    let e2 = parse_cmd_helper(cmd_iter)?;
                    Ok(Expr::Plus(Box::new(e1),Box::new(e2)))
                }
                else {
                    // println!("2");
                    Err(BeepboopError::SyntaxError)
                }
            }
            else {
                // println!("1");
                Err(BeepboopError::SyntaxError)
            }

        }
        Some("ting") => { // mult (times)
            if cmd_iter.next() == Some("clank") {
                let e1 = parse_cmd_helper(cmd_iter)?;
                if cmd_iter.next() == Some("clank") {
                    let e2 = parse_cmd_helper(cmd_iter)?;
                    Ok(Expr::Mult(Box::new(e1),Box::new(e2)))
                }
                else {
                    Err(BeepboopError::SyntaxError)
                }
            }
            else {
                Err(BeepboopError::SyntaxError)
            }

        }
        // Some("bip") => { // if
        //     if cmd_iter.next() == Some("clank") {
        //         let result = parse_if_then_else(&mut cmd_iter)?;
        //     }
        // }
        // Some("bop") => { // for
        //
        // }
        // Some("ding") => { // return
        //
        // }
        Some("clank") => { // open parenthesis
            parse_cmd_helper(cmd_iter)
        }
        // Some("clonk") => { // close parenthesis
       
        // }
        None => {
            println!("tried to parse empty string");
            Err(BeepboopError::SyntaxError)
        },
        _ => Err(BeepboopError::SyntaxError),
    }
}


pub struct BeepboopFile {
    pub lines: Vec<String>,
}

impl BeepboopFile {
    pub fn new(filename: String) -> Self {
        // let f = File::open(String::from("./code/var_assign.txt")).expect("File not found!");
        let f = File::open(filename).expect("File not found!");
        let f = BufReader::new(f);

        let mut lines: Vec<String> = Vec::new();
        for line in f.lines() {
            match line {
                Ok(l) => {
                    println!("[read] {}",l);
                    lines.push(l);

                },
                Err(error) => panic!("Couldn't read file! {}",error),
            }
        }
        BeepboopFile {lines}
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
