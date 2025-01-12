use crate::ast::Location;

#[derive(Debug, Clone, Copy)]
enum TokenKind {
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Quote,
    Symbol,
    String,
    Number,
    Comment,
}

#[derive(Debug, Clone, Copy)]
struct Token<'src> {
    kind: TokenKind,
    value: &'src str,
    location: Location,
}

impl<'src> Token<'src> {
    pub fn new(kind: TokenKind, value: &'src str, location: Location) -> Self {
        Self { kind, value, location }
    }
}

struct Lexer<'src> {
    source: &'src str,
    index: usize,
    line: u32,
    column: u32,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self { source, index: 0, line: 1, column: 1 }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn peek_char(lexer: &mut Lexer) -> Option<char> {
    lexer.source.chars().nth(lexer.index)
}

fn consume_char(lexer: &mut Lexer) -> Option<char> {
    match peek_char(lexer) {
        Some(ch) => {
            lexer.index += 1;
            Some(ch)
        }
        None => None,
    }
}
