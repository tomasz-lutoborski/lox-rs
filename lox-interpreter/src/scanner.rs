use crate::token::{Token, TokenType};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            source: source.to_string(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), self.line));
        self.tokens.clone()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_token(&mut self) {
        let binding = self.advance();
        let c = binding.as_str();

        match c {
            "(" => self.add_token(TokenType::LeftParen),
            ")" => self.add_token(TokenType::RightParen),
            "{" => self.add_token(TokenType::LeftBrace),
            "}" => self.add_token(TokenType::RightBrace),
            "," => self.add_token(TokenType::Comma),
            "." => self.add_token(TokenType::Dot),
            "-" => self.add_token(TokenType::Minus),
            "+" => self.add_token(TokenType::Plus),
            ";" => self.add_token(TokenType::Semicolon),
            "*" => self.add_token(TokenType::Star),
            "!" => {
                if self.match_char("=") {
                    self.add_token(TokenType::BangEqual)
                } else {
                    self.add_token(TokenType::Bang)
                }
            }
            "=" => {
                if self.match_char("=") {
                    self.add_token(TokenType::EqualEqual)
                } else {
                    self.add_token(TokenType::Equal)
                }
            }
            "<" => {
                if self.match_char("=") {
                    self.add_token(TokenType::LessEqual)
                } else {
                    self.add_token(TokenType::Less)
                }
            }
            ">" => {
                if self.match_char("=") {
                    self.add_token(TokenType::GreaterEqual)
                } else {
                    self.add_token(TokenType::Greater)
                }
            }
            "/" => {
                if self.match_char("/") {
                    while self.peek() != "\n" && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            " " | "\r" | "\t" => (),
            "\n" => self.line += 1,
            "\"" => self.string(),
            _ => todo!(),
        }
    }

    fn match_char(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected.chars().nth(0).unwrap() {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> String {
        if self.is_at_end() {
            return "\0".to_string();
        }
        self.source.chars().nth(self.current).unwrap().to_string()
    }

    fn string(&mut self) {
        while self.peek() != "\"" && !self.is_at_end() {
            if self.peek() == "\n" {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            todo!();
        }
        self.advance();
        let value = self
            .source
            .get(self.start + 1..self.current - 1)
            .expect("Source token is empty.")
            .to_string();
        self.add_token(TokenType::String(value));
    }

    fn advance(&mut self) -> String {
        self.current += 1;
        self.source
            .chars()
            .nth(self.current - 1)
            .unwrap()
            .to_string()
    }

    fn add_token(&mut self, tpe: TokenType) {
        let text = self
            .source
            .get(self.start..self.current)
            .expect("Source token is empty.")
            .to_string();
        self.tokens.push(Token::new(tpe, text, self.line))
    }
}
