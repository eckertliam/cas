use std::fmt::Display;
use crate::pos::Pos;

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
    value: f64,
    pos: Pos,
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    value: String,
    pos: Pos,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    exprs: Vec<Expression>,
    pos: Pos,
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.exprs.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(" "))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(Number),
    Symbol(Symbol),
    List(List),
}

impl Expression {
    pub fn new_number(value: f64, pos: Pos) -> Self {
        Self::Number(Number { value, pos })
    }

    pub fn pos(&self) -> Pos {
        match self {
            Expression::Number(n) => n.pos,
            Expression::Symbol(s) => s.pos,
            Expression::List(l) => l.pos,
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::List(l) => write!(f, "{}", l),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program(Vec<Expression>);

impl Default for Program {
    fn default() -> Self {
        Self(vec![])
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for e in &self.0 {
            write!(f, "{}\n", e)?;
        }
        Ok(())
    }
}
