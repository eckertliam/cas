use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::env::Env;

/// Objects are the core data type of the language
/// AST expressions are converted to Objects prior to eval and compilation
/// Objects are used within all passes of the interpreter and compiler
pub enum Object {
    Null,
    Bool(bool),
    Integer(i64),
    Float(f64),
    Rational {
        num: i64,
        den: i64,
    },
    String(String),
    Symbol(String),
    List(Rc<Vec<Object>>),
    Lambda {
        env: Rc<RefCell<Env>>,
        params: Vec<String>,
        body: Rc<Object>,
    },
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Null => write!(f, "null"),
            Object::Bool(b) => write!(f, "{}", b),
            Object::Integer(i) => write!(f, "{}", i),
            Object::Float(fl) => write!(f, "{}", fl),
            Object::Rational { num, den } => write!(f, "{} / {}", num, den),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Symbol(s) => write!(f, "{}", s),
            Object::List(l) => write!(f, "({})", l.iter().map(|e| e.to_string()).collect::<Vec<String>>().join(" ")),
            Object::Lambda { env, params, body } => write!(f, "(lambda ({}) {})", params.join(" "), body),
        }
    }
}