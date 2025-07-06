pub enum Expr {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Symbol(String),
    Pair(Box<Expr>, Box<Expr>),
}