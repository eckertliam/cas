use std::collections::VecDeque;
use std::fmt;

use crate::model::Model;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: TokenKind,
        location: Location,
    },
    UnexpectedEof {
        expected: String,
        location: Location,
    },
    InvalidNumber {
        text: String,
        location: Location,
    },
    UnterminatedString {
        location: Location,
    },
    UnmatchedParen {
        location: Location,
    },
    InvalidToken {
        location: Location,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found, location } => {
                write!(f, "Expected {} but found {:?} at line {}, column {}", 
                       expected, found, location.start_line, location.start_column)
            }
            ParseError::UnexpectedEof { expected, location } => {
                write!(f, "Unexpected end of file, expected {} at line {}, column {}", 
                       expected, location.start_line, location.start_column)
            }
            ParseError::InvalidNumber { text, location } => {
                write!(f, "Invalid number '{}' at line {}, column {}", 
                       text, location.start_line, location.start_column)
            }
            ParseError::UnterminatedString { location } => {
                write!(f, "Unterminated string at line {}, column {}", 
                       location.start_line, location.start_column)
            }
            ParseError::UnmatchedParen { location } => {
                write!(f, "Unmatched opening parenthesis at line {}, column {}", 
                       location.start_line, location.start_column)
            }
            ParseError::InvalidToken { location } => {
                write!(f, "Invalid token at line {}, column {}", 
                       location.start_line, location.start_column)
            }
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    Error,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Token {
    kind: TokenKind,
    span: Span,
    location: Location,
}

impl<'src> Token {
    pub fn new(
        kind: TokenKind,
        start: usize,
        end: usize,
        start_line: u32,
        start_column: u32,
        end_line: u32,
        end_column: u32,
    ) -> Self {
        Self {
            kind,
            span: Span { start, end },
            location: Location {
                start_line,
                start_column,
                end_line,
                end_column,
            },
        }
    }

    pub fn text(&self, source: &'src str) -> &'src str {
        &source[self.span.start..self.span.end]
    }
}

fn is_symbol_char(byte: u8) -> bool {
    byte.is_ascii_alphanumeric()
        || byte == b'_'
        || byte == b'?'
        || byte == b'!'
        || byte == b'='
        || byte == b'<'
        || byte == b'>'
        || byte == b'-'
        || byte == b'+'
        || byte == b'*'
        || byte == b'/'
        || byte == b'%'
        || byte == b'&'
        || byte == b'|'
        || byte == b'~'
        || byte == b'#'
}

struct Lexer<'src> {
    bytes: &'src [u8],
    start: usize,
    current: usize,
    line: u32,
    column: u32,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            bytes: source.as_bytes(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn collect(&mut self) -> Result<Vec<Token>, ParseError> {
        let mut tokens = Vec::new();
        while self.peek() != b'\0' {
            let token = self.next()?;
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn simple_token(&self, kind: TokenKind) -> Token {
        self.make_token(kind, self.line, self.column)
    }

    fn make_token(&self, kind: TokenKind, start_line: u32, start_column: u32) -> Token {
        Token {
            kind,
            span: Span {
                start: self.start,
                end: self.current,
            },
            location: Location {
                start_line,
                start_column,
                end_line: self.line,
                end_column: self.column,
            },
        }
    }

    fn peek(&self) -> u8 {
        if self.current < self.bytes.len() {
            self.bytes[self.current]
        } else {
            b'\0'
        }
    }

    fn consume(&mut self) -> u8 {
        let byte = self.peek();
        self.current += 1;
        if byte == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        byte
    }

    fn skip_whitespace(&mut self) {
        while self.peek().is_ascii_whitespace() {
            self.consume();
        }
    }

    fn skip_comment(&mut self) {
        while self.peek() != b'\n' && self.peek() != b'\0' {
            self.consume();
        }
        if self.peek() == b'\n' {
            self.consume();
        }
    }

    fn next(&mut self) -> Result<Token, ParseError> {
        self.skip_whitespace();
        self.start = self.current;

        let start_line = self.line;
        let start_column = self.column;

        let byte = self.consume();

        match byte {
            b'(' => Ok(self.simple_token(TokenKind::LParen)),
            b')' => Ok(self.simple_token(TokenKind::RParen)),
            b'[' => Ok(self.simple_token(TokenKind::LBracket)),
            b']' => Ok(self.simple_token(TokenKind::RBracket)),
            b'{' => Ok(self.simple_token(TokenKind::LBrace)),
            b'}' => Ok(self.simple_token(TokenKind::RBrace)),
            b'\'' => Ok(self.simple_token(TokenKind::Quote)),
            b'"' => self.string_token(start_line, start_column),
            b';' => {
                self.skip_comment();
                self.next()
            }
            b'0'..=b'9' => Ok(self.number_token(start_line, start_column)),
            b if is_symbol_char(b) => Ok(self.symbol_token(start_line, start_column)),
            b'\0' => Ok(self.simple_token(TokenKind::Eof)),
            _ => Err(ParseError::InvalidToken {
                location: Location {
                    start_line,
                    start_column,
                    end_line: self.line,
                    end_column: self.column,
                },
            }),
        }
    }

    fn string_token(&mut self, start_line: u32, start_column: u32) -> Result<Token, ParseError> {
        while self.peek() != b'"' && self.peek() != b'\0' {
            self.consume();
        }
        if self.peek() == b'\0' {
            return Err(ParseError::UnterminatedString {
                location: Location {
                    start_line,
                    start_column,
                    end_line: self.line,
                    end_column: self.column,
                },
            });
        }
        self.consume(); // consume closing quote
        Ok(self.make_token(TokenKind::String, start_line, start_column))
    }

    fn number_token(&mut self, start_line: u32, start_column: u32) -> Token {
        while self.peek().is_ascii_digit() {
            self.consume();
        }

        if self.peek() == b'.' {
            self.consume();
            while self.peek().is_ascii_digit() {
                self.consume();
            }
        }

        self.make_token(TokenKind::Number, start_line, start_column)
    }

    fn symbol_token(&mut self, start_line: u32, start_column: u32) -> Token {
        while is_symbol_char(self.peek()) {
            self.consume();
        }
        self.make_token(TokenKind::Symbol, start_line, start_column)
    }
}

struct Parser<'src> {
    tokens: Vec<Token>,
    source: &'src str,
    idx: usize,
    stack: VecDeque<Token>,
}

impl<'src> Parser<'src> {
    pub fn new(source: &'src str) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.collect()?;
        Ok(Self {
            tokens,
            source,
            idx: 0,
            stack: VecDeque::new(),
        })
    }

    pub fn parse(&mut self) -> Result<Vec<Model>, ParseError> {
        let mut exprs = Vec::new();
        
        while !self.is_at_end() {
            exprs.push(self.parse_expr()?);
        }
        
        // Check for unmatched parentheses
        if let Some(unmatched) = self.stack.front() {
            return Err(ParseError::UnmatchedParen {
                location: unmatched.location,
            });
        }
        
        Ok(exprs)
    }
    
    fn is_at_end(&self) -> bool {
        self.idx >= self.tokens.len() || self.current_token().kind == TokenKind::Eof
    }
    
    fn current_token(&self) -> Token {
        if self.idx < self.tokens.len() {
            self.tokens[self.idx]
        } else {
            // Return EOF token with last position
            let last_line = self.tokens.last().map(|t| t.location.end_line).unwrap_or(1);
            let last_column = self.tokens.last().map(|t| t.location.end_column).unwrap_or(1);
            Token::new(TokenKind::Eof, 0, 0, last_line, last_column, last_line, last_column)
        }
    }
    
    fn advance(&mut self) -> Token {
        let token = self.current_token();
        if self.idx < self.tokens.len() {
            self.idx += 1;
        }
        token
    }
    
    fn expect(&mut self, expected: TokenKind) -> Result<Token, ParseError> {
        let token = self.current_token();
        if token.kind == expected {
            Ok(self.advance())
        } else if token.kind == TokenKind::Eof {
            Err(ParseError::UnexpectedEof {
                expected: format!("{:?}", expected),
                location: token.location,
            })
        } else {
            Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", expected),
                found: token.kind,
                location: token.location,
            })
        }
    }
    
    fn parse_expr(&mut self) -> Result<Model, ParseError> {
        if self.is_at_end() {
            return Err(ParseError::UnexpectedEof {
                expected: "expression".to_string(),
                location: self.current_token().location,
            });
        }
        
        let token = self.current_token();
        
        match token.kind {
            TokenKind::Number => self.parse_number(),
            TokenKind::String => self.parse_string(),
            TokenKind::Symbol => self.parse_symbol(),
            TokenKind::LParen => self.parse_list(),
            TokenKind::Quote => self.parse_quote(),
            TokenKind::Eof => Err(ParseError::UnexpectedEof {
                expected: "expression".to_string(),
                location: token.location,
            }),
            _ => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                found: token.kind,
                location: token.location,
            }),
        }
    }
    
    fn parse_number(&mut self) -> Result<Model, ParseError> {
        let token = self.advance();
        let text = token.text(self.source);
        
        if text.contains('.') {
            text.parse::<f64>()
                .map(Model::Float)
                .map_err(|_| ParseError::InvalidNumber {
                    text: text.to_string(),
                    location: token.location,
                })
        } else {
            text.parse::<i64>()
                .map(Model::Int)
                .map_err(|_| ParseError::InvalidNumber {
                    text: text.to_string(),
                    location: token.location,
                })
        }
    }
    
    fn parse_string(&mut self) -> Result<Model, ParseError> {
        let token = self.advance();
        let text = token.text(self.source);
        
        // Remove surrounding quotes
        if text.len() >= 2 {
            let content = &text[1..text.len()-1];
            Ok(Model::String(content.to_string()))
        } else {
            Err(ParseError::UnterminatedString {
                location: token.location,
            })
        }
    }
    
    fn parse_symbol(&mut self) -> Result<Model, ParseError> {
        let token = self.advance();
        let text = token.text(self.source);
        
        Ok(match text {
            "#t" => Model::Bool(true),
            "#f" => Model::Bool(false),
            "null" => Model::Null,
            _ => Model::Symbol(text.to_string())
        })
    }
    
    fn parse_list(&mut self) -> Result<Model, ParseError> {
        let open_paren = self.expect(TokenKind::LParen)?;
        self.stack.push_back(open_paren);
        
        let mut elements = Vec::new();
        
        while !self.is_at_end() && self.current_token().kind != TokenKind::RParen {
            elements.push(self.parse_expr()?);
        }
        
        self.expect(TokenKind::RParen)?;
        self.stack.pop_back();
        
        // Convert Vec<Expr> to right-associative nested pairs
        // Empty list becomes Null
        if elements.is_empty() {
            Ok(Model::Null)
        } else {
            let mut result = Model::Null;
            for expr in elements.into_iter().rev() {
                result = Model::Pair(Box::new(expr), Box::new(result));
            }
            Ok(result)
        }
    }
    
    fn parse_quote(&mut self) -> Result<Model, ParseError> {
        self.advance(); // consume quote
        
        let expr = self.parse_expr()?;
        // In Scheme, 'x is equivalent to (quote x)
        // We'll represent this as a pair: (quote . (x . null))
        let quote_symbol = Model::Symbol("quote".to_string());
        let quoted_expr = Model::Pair(Box::new(expr), Box::new(Model::Null));
        Ok(Model::Pair(Box::new(quote_symbol), Box::new(quoted_expr)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number() {
        let mut parser = Parser::new("123").unwrap();
        let exprs = parser.parse().unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(exprs[0], Model::Int(123));
    }

    #[test]
    fn test_parse_string() {
        let mut parser = Parser::new("\"Hello, World!\"").unwrap();
        let exprs = parser.parse().unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(exprs[0], Model::String("Hello, World!".to_string()));
    }
    
    #[test]
    fn test_parse_s_expr() {
        let mut parser = Parser::new("(1 2 3)").unwrap();
        let exprs = parser.parse().unwrap();
        assert_eq!(exprs.len(), 1);
        assert_eq!(exprs[0], Model::Pair(Box::new(Model::Int(1)), Box::new(Model::Pair(Box::new(Model::Int(2)), Box::new(Model::Pair(Box::new(Model::Int(3)), Box::new(Model::Null)))))));
    }
}