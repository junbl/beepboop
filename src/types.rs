// pub mod types {
// #[derive(Copy, Clone)]
pub enum Expr {
    Assign(String,Box<Expr>),
    Lookup(String),
    Const(i32),
    Plus(Box<Expr>,Box<Expr>),
    Mult(Box<Expr>,Box<Expr>),
}
#[derive(Copy, Clone)]
pub enum Value {
    Num(i32),
    Bin(bool),
    // Str(String),
}
// }
