use std::error::Error;
use std::fmt;

// pub mod types {
// #[derive(Copy, Clone)]
#[derive(PartialEq)]
pub enum Expr {
    Assign(String,Box<Expr>),
    Lookup(String),
    Const(i32),
    Plus(Box<Expr>,Box<Expr>),
    Mult(Box<Expr>,Box<Expr>),
    // IfThenElse(Box<Expr>,Box<Expr>,Box<Expr>),
    // Expr::And(expr1, expr2)
    // Expr::Or(expr1, expr2)
    // Expr::Equals(expr1, expr2)
    // Expr::Greater(expr1, expr2)
    // Expr::Less(expr1, expr2)
    // Expr::IfThenElse(condition, if_true, if_false)
    // Expr::For(iter, range, expr)
    // Expr::Function(arg, body, state)
    // Expr::Fold(initial, list, function)
}
// pub enum List<T> {
//     Empty,
//     Cons(Expr,Box<List<T>>),
// }
#[derive(Copy, Clone,Debug,PartialEq)]
pub enum Value {
    Num(i32),
    // Bin(bool),
    // Str(String),
}
#[derive(Debug,PartialEq)]
pub enum BeepboopError {
    ParseError,
    SyntaxError,
}

#[derive(Debug)]
pub enum ParseResult<T, U, V> {
    Ok(T),
    Continue(U),
    Err(V),
}

impl Error for BeepboopError {
    // fn source(&self) -> Option<&(dyn Error + 'static)> {
    //     Some(
    // }
}

impl fmt::Display for BeepboopError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeepboopError::ParseError => write!(f, "Parse error!"),
            BeepboopError::SyntaxError => write!(f, "Syntax error!"),
        }
    }
}
// }
