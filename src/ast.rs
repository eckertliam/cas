use std::{fmt::Display, rc::Rc};

pub enum ExprKind {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Symbol(String),
    List(Rc<Vec<Expr>>),
}

impl Display for ExprKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprKind::Null => write!(f, "null"),
            ExprKind::Bool(b) => write!(f, "{}", b),
            ExprKind::Integer(i) => write!(f, "{}", i),
            ExprKind::Float(fl) => write!(f, "{}", fl),
            ExprKind::String(s) => write!(f, "\"{}\"", s),
            ExprKind::Symbol(s) => write!(f, "{}", s),
            ExprKind::List(l) => write!(f, "({})", l.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(" ")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

/// Expressions are the nodes of the AST
/// They have a location attached that is later stripped away
/// Expressions are converted to Objects to be evaluated and compiled
pub struct Expr {
    pub kind: ExprKind,
    pub loc: Location,
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}