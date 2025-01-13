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
    Error,
    Eof,
}

#[derive(Debug, Clone, Copy)]
struct Token<'src> {
    kind: TokenKind,
    value: &'src str,
    location: Location,
}

impl<'src> Token<'src> {
    pub fn new(kind: TokenKind, value: &'src str, location: Location) -> Self {
        Self {
            kind,
            value,
            location,
        }
    }
}

struct Lexer<'src> {
    source: &'src str,
    start: usize,
    current: usize,
    line: u32,
    column: u32,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        skip_whitespace(self);
        self.start = self.current;

        let ch = match consume_char(self) {
            Some(ch) => ch,
            None => return Some(Token::new(TokenKind::Eof, "", Location { line: self.line, column: self.column })),
        };

        match ch {
            '(' => Some(take_token(self, TokenKind::LParen)),
            ')' => Some(take_token(self, TokenKind::RParen)),
            '[' => Some(take_token(self, TokenKind::LBracket)),
            ']' => Some(take_token(self, TokenKind::RBracket)),
            '{' => Some(take_token(self, TokenKind::LBrace)),
            '}' => Some(take_token(self, TokenKind::RBrace)),
            '\'' => Some(take_token(self, TokenKind::Quote)),
            ';' => take_comment(self),
            '"' => take_string(self),
            '0'..='9' => take_number(self),
            ch if is_symbol_char(ch) => take_symbol(self),
            _ => take_error(self, "Unexpected character"),
        }
    }
}

fn peek_char<'src>(lexer: &mut Lexer<'src>) -> Option<char> {
    lexer.source.chars().nth(lexer.current)
}

fn consume_char<'src>(lexer: &mut Lexer<'src>) -> Option<char> {
    match peek_char(lexer) {
        Some(ch) => {
            lexer.current += 1;
            Some(ch)
        }
        None => None,
    }
}

fn take_lexeme<'src>(lexer: &mut Lexer<'src>) -> &'src str {
    let start = lexer.start;
    let end = lexer.current;
    &lexer.source[start..end]
}

fn take_token<'src>(lexer: &mut Lexer<'src>, kind: TokenKind) -> Token<'src> {
    Token::new(
        kind,
        take_lexeme(lexer),
        Location {
            line: lexer.line,
            column: lexer.column,
        },
    )
}

fn skip_whitespace<'src>(lexer: &mut Lexer<'src>) {
    while let Some(ch) = peek_char(lexer) {
        if ch.is_whitespace() {
            consume_char(lexer);
        } else {
            break;
        }
    }
}

fn take_comment<'src>(lexer: &mut Lexer<'src>) -> Option<Token<'src>> {
    // skip over semicolons
    while let Some(ch) = peek_char(lexer) {
        if ch == ';' {
            consume_char(lexer);
        } else {
            break;
        }
    }
    // junk token
    let _ = take_token(lexer, TokenKind::Comment);
    lexer.start = lexer.current;
    // then take to the end of the line
    while let Some(ch) = peek_char(lexer) {
        if ch == '\n' {
            break;
        }
        consume_char(lexer);
    }
    // comment token
    Some(take_token(lexer, TokenKind::Comment))
}

fn take_string<'src>(lexer: &mut Lexer<'src>) -> Option<Token<'src>> {
    while let Some(ch) = peek_char(lexer) {
        if ch == '"' {
            break;
        } else if ch == '\\' {
            // skip over the escape character
            consume_char(lexer);
            // consume the next character
            consume_char(lexer);
        } else {
            consume_char(lexer);
        }
    }
    // trim quotes
    let lexeme = take_lexeme(lexer);
    let trimmed = &lexeme[1..lexeme.len() - 1];
    Some(Token::new(
        TokenKind::String,
        trimmed,
        Location {
            line: lexer.line,
            column: lexer.column,
        },
    ))
}

fn take_number<'src>(lexer: &mut Lexer<'src>) -> Option<Token<'src>> {
    while let Some(ch) = peek_char(lexer) {
        if ch.is_digit(10) {
            consume_char(lexer);
        } else {
            break;
        }
    }
    if let Some('.') = peek_char(lexer) {
        consume_char(lexer);
        while let Some(ch) = peek_char(lexer) {
            if ch.is_digit(10) {
                consume_char(lexer);
            } else {
                break;
            }
        }
    }
    Some(take_token(lexer, TokenKind::Number))
}

const SYMBOL_CHARS: [char; 14] = [
    '!', '?', '/', '=', '+', '-', '*', '%', '<', '>', '&', '|', '#', '_'
];

fn is_symbol_char(ch: char) -> bool {
    SYMBOL_CHARS.contains(&ch) || ch.is_alphanumeric() 
}

fn take_symbol<'src>(lexer: &mut Lexer<'src>) -> Option<Token<'src>> {
    while let Some(ch) = peek_char(lexer) {
        if is_symbol_char(ch) || ch.is_digit(10) {
            consume_char(lexer);
        } else {
            break;
        }
    }
    Some(take_token(lexer, TokenKind::Symbol))
}

fn take_error<'src>(lexer: &mut Lexer<'src>, message: &'src str) -> Option<Token<'src>> {
    Some(Token::new(TokenKind::Error, message, Location { line: lexer.line, column: lexer.column }))
}
