use std::iter::Peekable;
use std::str::Chars;

use crate::ast::{Expression, Program};
use crate::pos::Pos;

struct Reader<'src> {
    source: Peekable<Chars<'src>>,
    paren_depth: usize,
    program: Vec<Expression>,
    line: usize,
    column: usize,
}

impl<'src> Reader<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source: source.chars().peekable(),
            paren_depth: 0,
            program: Vec::new(),
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

    fn read_symbol(&mut self) -> Result<Expression, String> {
        let pos = self.pos();
        let mut symbol = String::new();
        while let Some(&c) = self.peek() {
            if is_symbol_char(c) {
                symbol.push(c);
                self.consume();
            } else {
                break;
            }
        }
        Ok(Expression::new_symbol(symbol, pos))
    }


    fn read_list(&mut self) -> Result<Expression, String> {
        let pos = self.pos();
        self.consume();
        let mut exprs = Vec::new();
        self.push();
        while let Some(&c) = self.peek() {
            if c == ')' {
                self.pop()?;
                self.consume();
                break;
            }
            exprs.push(self.read_expression()?);
        }
        Ok(Expression::new_list(exprs, pos))
    }

    fn read_expression(&mut self) -> Result<Expression, String> {
        self.skip_whitespace();
        match self.peek() {
            Some(ch) => match ch {
                '(' => self.read_list(),
                c if is_symbol_char(*c) => self.read_symbol(),
                c if c.is_digit(10) => self.read_number(),
                _ => Err(format!("Invalid expression at {}:{}", self.line, self.column)),
            },
            None => Err(format!("Unexpected end of file at {}:{}", self.line, self.column)),
        }
    }

    pub fn read(&mut self) -> Result<Program, String> {
        while let Some(_) = self.peek() {
            let expr = self.read_expression()?;
            self.program.push(expr);
        }
        Ok(Program::new(&self.program))
    }
}

static SYMBOL_CHARS: [char; 12] = ['!', '?', '+', '-', '*', '/', '=', '<', '>', '&', '|', '%'];

fn is_symbol_char(c: char) -> bool {
    c.is_alphanumeric() || SYMBOL_CHARS.contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_list() {
        let mut reader = Reader::new("(1 2 3)");
        let program = reader.read().unwrap();
        assert_eq!(program.to_string(), "(1 2 3)\n");
    }

    #[test]
    fn test_symbol() {
        let mut reader = Reader::new("(a b c)");
        let program = reader.read().unwrap();
        assert_eq!(program.to_string(), "(a b c)\n");
    }
}
