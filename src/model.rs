#[derive(Debug, PartialEq)]
pub enum Model {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
    Symbol(String),
    Pair(Box<Model>, Box<Model>),
}