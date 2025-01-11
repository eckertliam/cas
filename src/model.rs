pub struct Cons {
    pub car: Box<Expr>,
    pub cdr: Box<Expr>,
}

pub enum Expr {
    Cons(Cons),
    Nil,
    Int(i64),
    Bool(bool),
    String(String),
    Lambda(Lambda),
    Symbol(String),
    Quote(Box<Expr>),
    If {
        cond: Box<Expr>,
        then: Box<Expr>,
        _else: Box<Expr>,
    },
    Define {
        name: String,
        value: Box<Expr>,
    },
    Set {
        name: String,
        value: Box<Expr>,
    },
    Begin(Vec<Box<Expr>>),
    Let {
        bindings: Vec<(String, Box<Expr>)>,
        body: Box<Expr>,
    },
}

pub struct Lambda {
    pub params: Vec<String>,
    pub body: Box<Expr>,
}
