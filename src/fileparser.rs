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
            _other => break,
        }
    }
    Ok(total)
}

pub fn parse_n_args<'a, I>(cmd_iter: &mut I, n: i32) -> Result<Vec<Expr>, BeepboopError>
where
    I: Iterator<Item = &'a str>,
{
    let mut parsed_exprs: Vec<Expr> = Vec::new();
    for _ in 0..n {
        if cmd_iter.next() == Some("clank") {
            parsed_exprs.push(parse_cmd_helper(cmd_iter)?);
        }
        else {
            return Err(BeepboopError::SyntaxError)
        }
    }
    Ok(parsed_exprs)
}


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
            println!("number");
            let target_num: i32 = parse_num(cmd_iter)?;
            Ok(Expr::Const(target_num))
        }
        // Some("beep") => { // numbers
        //     let target_num: i32 = (parse_num(&mut cmd_iter)? << 1) + 1;
        //     Ok(Expr::Const(target_num))
        // }
        Some("whirr") => { // variable assignment
            println!("assign");
            if let Some(var_name) = cmd_iter.next() {
                let target_expr = parse_cmd_helper(cmd_iter)?;
                Ok(Expr::Assign(var_name.to_string(),
                    Box::new(target_expr)))
            }
            else {
                Err(BeepboopError::ParseError)
            }
        },
        Some("brrring") => {
            println!("lookup");
            if let Some(var_name) = cmd_iter.next() {
                Ok(Expr::Lookup(var_name.to_string()))
            }
            else {
                Err(BeepboopError::ParseError)
            }
        }
        Some("plop") => { // plus
            println!("plus");
            if cmd_iter.next() == Some("clank") {
                let e1 = parse_cmd_helper(cmd_iter)?;
                if cmd_iter.next() == Some("clank") {
                    let e2 = parse_cmd_helper(cmd_iter)?;
                    Ok(Expr::Plus(Box::new(e1),Box::new(e2)))
                }
                else {
                    Err(BeepboopError::SyntaxError)
                }
            }
            else {
                Err(BeepboopError::SyntaxError)
            }

        }
        // Some("plop") => {
        //     let args = parse_n_args(cmd_iter, 2)?;
        //     let args = args.into_iter();

        //     Ok(Expr::Plus(Box::new(args.next()),Box::new(args.next())))
        // }
        Some("ting") => { // mult (times)
            println!("mult");
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
        Some("bip") => { // if
            println!("if");
            if cmd_iter.next() == Some("clank") {
                let econdition = parse_cmd_helper(cmd_iter)?;
                if cmd_iter.next() == Some("clank") {
                    let et = parse_cmd_helper(cmd_iter)?;

                    if cmd_iter.next() == Some("clank") {
                        let ef = parse_cmd_helper(cmd_iter)?;

                        Ok(Expr::IfThenElse(Box::new(econdition),Box::new(et),Box::new(ef)))
                    }
                    else {
                        Err(BeepboopError::SyntaxError)
                    }
                }
                else {
                    Err(BeepboopError::SyntaxError)
                }
            }
            else {
                Err(BeepboopError::SyntaxError)
            }
        }
        Some("ratatat") => { // for
            // do e_body e_loops times
            println!("for");
            if cmd_iter.next() == Some("clank") {
                let e_loops = parse_cmd_helper(cmd_iter)?;
                if cmd_iter.next() == Some("clank") {
                    let e_body = parse_cmd_helper(cmd_iter)?;
                    Ok(Expr::For(Box::new(e_loops),Box::new(e_body)))
                }
                else {
                    Err(BeepboopError::SyntaxError)
                }
            }
            else {
                Err(BeepboopError::SyntaxError)
            }

       
        }
        // Some("ding") => { // return
        //
        // }
        Some("clank") => { // open parenthesis
            parse_cmd_helper(cmd_iter)
        }
        // Some("clonk") => { // close parenthesis
       
        // }
        None => {
            eprintln!("tried to parse empty string");
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
