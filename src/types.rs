pub mod types {
    pub enum Expr {
        Assign(String,Expr),
        Lookup(String),
        Const(i32),
        Plus(Expr,Expr),
        Mult(Expr,Expr),
    }
    pub enum Value {
        Num(i32),
        Str(String),
    }
}
