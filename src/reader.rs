use std::iter::Peekable;
use std::str::Chars;

use crate::ast::{Expression, Program};
use crate::pos::Pos;

struct Reader<'src> {
    source: Peekable<Chars<'src>>,
    paren_depth: usize,
    program: Program,
    line: usize,
    column: usize,
}

impl<'src> Reader<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source: source.chars().peekable(),
            paren_depth: 0,
            program: Program::default(),
            line: 1,
            column: 1,
        }
    }

    // pop a parenthesis
    fn pop(&mut self) -> Result<(), String> {
        if self.paren_depth == 0 {
            return Err(format!("Unexpected closing parenthesis at {}:{}", self.line, self.column));
        }
        self.paren_depth -= 1;
        Ok(())
    }

    // push a parenthesis
    fn push(&mut self) {
        self.paren_depth += 1;
    }

    // peek a character
    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    // next character
    fn consume(&mut self) -> Option<char> {
        let c = self.source.next()?;
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(c)
    }

    // current position
    fn pos(&self) -> Pos {
        Pos::new(self.line, self.column)
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> Result<Expression, String> {
        let pos = self.pos();
        let mut num_str = String::new();
        while let Some(&c) = self.peek() {
            if c.is_digit(10) {
                num_str.push(c);
                self.consume();
            } else {
                break;
            }
        }
        if let Some('.') = self.peek() {
            num_str.push('.');
            self.consume();
            while let Some(&c) = self.peek() {
                if c.is_digit(10) {
                    num_str.push(c);
                    self.consume();
                } else {
                    break;
                }
            }
        }
        if let Ok(value) = num_str.parse::<f64>() {
            Ok(Expression::new_number(value, pos))
        } else {
            Err(format!("Invalid number at {}:{}", pos.line, pos.column))
        }
    }
}
