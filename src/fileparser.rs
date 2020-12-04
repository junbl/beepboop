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

// pub fn parse_n_args<'a, I>(cmd_iter: &mut I, n: i32) -> Result<Vec<Expr>, BeepboopError>
// where
//     I: Iterator<Item = &'a str>,
// {
//     let mut parsed_exprs: Vec<Expr> = Vec::new();
//     for _ in 0..n {
//         if cmd_iter.next() == Some("clank") {
//             parsed_exprs.push(parse_cmd_helper(cmd_iter)?);
//         }
//         else {
//             return Err(BeepboopError::SyntaxError)
//         }
//     }
//     Ok(parsed_exprs)
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
            println!("number: {}",target_num);
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
            let e1 = parse_cmd_helper(cmd_iter)?;
            let e2 = parse_cmd_helper(cmd_iter)?;
            Ok(Expr::Plus(Box::new(e1),Box::new(e2)))

        }
        Some("ting") => { // mult (times)
            println!("mult");
            let e1 = parse_cmd_helper(cmd_iter)?;
            let e2 = parse_cmd_helper(cmd_iter)?;
            Ok(Expr::Mult(Box::new(e1),Box::new(e2)))
        }
        Some("boing") => { // negate
            println!("negate");
            let e = parse_cmd_helper(cmd_iter)?;
            Ok(Expr::Negate(Box::new(e)))
        }
        Some("zeep") => { // greater than
            println!("greater");
            let e1 = parse_cmd_helper(cmd_iter)?;
            let e2 = parse_cmd_helper(cmd_iter)?;
            Ok(Expr::Greater(Box::new(e1),Box::new(e2)))
        }
        Some("zip") => { // less than
            println!("less");
            let e1 = parse_cmd_helper(cmd_iter)?;
            let e2 = parse_cmd_helper(cmd_iter)?;
            Ok(Expr::Less(Box::new(e1),Box::new(e2)))
        }
        Some("zap") => { // and
            println!("and");
            let e1 = parse_cmd_helper(cmd_iter)?;
            let e2 = parse_cmd_helper(cmd_iter)?;
            Ok(Expr::And(Box::new(e1),Box::new(e2)))
        }
        Some("zorp") => { // or
            println!("or");
            let e1 = parse_cmd_helper(cmd_iter)?;
            let e2 = parse_cmd_helper(cmd_iter)?;
            Ok(Expr::Or(Box::new(e1),Box::new(e2)))
        }
        Some("bzz") => { // equal
            println!("equal");
            let e1 = parse_cmd_helper(cmd_iter)?;
            let e2 = parse_cmd_helper(cmd_iter)?;
            Ok(Expr::Equal(Box::new(e1),Box::new(e2)))
        }
        Some("bip") => { // if
            println!("if");
            let econdition = parse_cmd_helper(cmd_iter)?;
            let et = parse_cmd_helper(cmd_iter)?;
            let ef = parse_cmd_helper(cmd_iter)?;

            Ok(Expr::IfThenElse(Box::new(econdition),Box::new(et),Box::new(ef)))
        }
        Some("ratatat") => { // for
            // do e_body e_loops times
            println!("for");
            let e_loops = parse_cmd_helper(cmd_iter)?;
            let e_body = parse_cmd_helper(cmd_iter)?;
            Ok(Expr::For(Box::new(e_loops),Box::new(e_body)))
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
            Err(BeepboopError::ParseError)
        },
        _ => Err(BeepboopError::ParseError),
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
