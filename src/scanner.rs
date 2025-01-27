use crate::error::LoxError;
use crate::token::*;
use crate::token_type::TokenType;

pub struct Scanner {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(mut e) => {
                    e.report("".to_string());
                    had_error = Some(e);
                }
            };
        }

        self.tokens.push(Token::eof(self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                let tok = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(tok);
            }
            '=' => {
                let tok = if self.is_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(tok);
            }
            '<' => {
                let tok = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(tok);
            }
            '>' => {
                let tok = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(tok);
            }
            '/' => {
                if self.is_match('/') {
                    while self.peak() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            _ => {
                return Err(LoxError::error(
                    self.line,
                    "Unexpected character.".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current += 1;
        result
    }

    fn add_token(&mut self, kind: TokenType) {
        self.add_token_object(kind, None);
    }

    fn add_token_object(&mut self, kind: TokenType, literal: Option<Object>) {
        let lexeme = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(kind, lexeme, literal, self.line))
    }

    fn is_match(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn peak(&mut self) -> char {
        match self.source.get(self.current) {
            Some(_) if self.is_at_end() => {
                return '\0';
            }
            _ => return *self.source.get(self.current).unwrap(),
        }
    }

    fn string(&mut self) {
        while self.peak() != '"' && !self.is_at_end() {
            if self.peak() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            LoxError::error(self.line, "Unterminated string.".to_string());
            return;
        }

        self.advance();

        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_object(TokenType::String, Some(Object::Str(value)))
    }
}
