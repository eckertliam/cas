use std::fmt::{Display, Debug};

#[derive(Clone, Copy, PartialEq)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
}

impl Pos {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Default for Pos {
    fn default() -> Self {
        Self::new(1, 1)
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
