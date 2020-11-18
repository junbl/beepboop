// pub mod types {
pub enum Expr {
    Assign(String,Box<Expr>),
    Lookup(String),
    Const(i32),
    Plus(Box<Expr>,Box<Expr>),
    Mult(Box<Expr>,Box<Expr>),
}
pub enum Value {
    Num(i32),
    Str(String),
}
// }
