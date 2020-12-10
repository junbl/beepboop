use std::error::Error;
use std::fmt;

// pub mod types {
#[derive(PartialEq, Clone)]
pub enum Expr {
    Assign(String, Box<Expr>),
    Lookup(String),
    Const(i32),
    Plus(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Negate(Box<Expr>),
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),
    For(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Equal(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    // Expr::Function(arg, body, state)
    // Expr::Fold(initial, list, function)
}
// pub enum List<T> {
//     Empty,
//     Cons(Expr,Box<List<T>>),
// }

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Value {
    Num(i32),
    Bin(bool),
}

#[derive(Debug, PartialEq)]
pub enum BeepboopError {
    ParseError,
    SyntaxError,
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
