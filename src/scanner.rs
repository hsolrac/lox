use crate::error::*;
use crate::token::*;

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_tokens();
        }

        self.tokens.push(Token::eof(self.line));
        Ok(&self.tokens)
    }

    pub fn is_at_end(self) -> bool {
        self.current >= self.source.len()
    }
}
