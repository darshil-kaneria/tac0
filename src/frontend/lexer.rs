// src/frontend/lexer.rs (rust)
#[derive(Debug, Clone)]
pub enum Token {
    Number(i64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Semi,
}

pub struct Lexer<'input> {
    input: &'input str,
    pos: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if self.pos >= self.input.len() {
            return None;
        }
        let ch = self.current_char();
        let token = match ch {
            '+' => { self.pos += 1; Some(Token::Plus) },
            '-' => { self.pos += 1; Some(Token::Minus) },
            '*' => { self.pos += 1; Some(Token::Star) },
            '/' => { self.pos += 1; Some(Token::Slash) },
            '(' => { self.pos += 1; Some(Token::LParen) },
            ')' => { self.pos += 1; Some(Token::RParen) },
            ';' => { self.pos += 1; Some(Token::Semi) },
            '0'..='9' => Some(self.lex_number()),
            _ if ch.is_alphabetic() || ch == '_' => Some(self.lex_ident()),
            _ => { self.pos += 1; self.next_token() },
        };
        token
    }

    fn current_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.current_char();
            if ch.is_whitespace() {
                self.pos += ch.len_utf8();
            } else {
                break;
            }
        }
    }

    fn ignore_comments(&mut self) {
        if self.input[self.pos..].starts_with("//") {
            while self.pos < self.input.len() && self.current_char() != '\n' {
                self.pos += 1;
            }
        }
    }

    fn lex_number(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() && self.current_char().is_digit(10) {
            self.pos += 1;
        }
        let num_str = &self.input[start..self.pos];
        let n = num_str.parse().unwrap();
        Token::Number(n)
    }

    fn lex_ident(&mut self) -> Token {
        let start = self.pos;
        while self.pos < self.input.len() &&
            (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.pos += 1;
        }
        let ident = self.input[start..self.pos].to_string();
        Token::Ident(ident)
    }
}