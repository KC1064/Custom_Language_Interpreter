#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Assume,
    Eq,
    Add,
    Sub,
    Mul,
    Div,
    Number(i64),
    Identifier(String),
    EndOfFile,
    Invalid,
}

pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0 }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.pos >= self.input.len() {
            return Token::EndOfFile;
        }

        let c = self.current_char();
        match c {
            'a' => {
                if self.matches("assume") {
                    return Token::Assume;
                }
                Token::Invalid
            }
            'e' => {
                if self.matches("eq") {
                    return Token::Eq;
                }
                Token::Invalid
            }
            '+' => {
                self.advance();
                return Token::Add;
            }
            '-' => {
                self.advance();
                return Token::Sub;
            }
            '*' => {
                self.advance();
                return Token::Mul;
            }
            '/' => {
                self.advance();
                return Token::Div;
            }
            '0'..='9' => return self.number(),
            'a'..='z' | 'A'..='Z' => return self.identifier(),
            _ => Token::Invalid,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.current_char().is_whitespace() {
            self.advance();
        }
    }

    fn current_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn advance(&mut self) {
        self.pos += self.current_char().len_utf8();
    }

    fn matches(&mut self, keyword: &str) -> bool {
        if self.input[self.pos..].starts_with(keyword) {
            self.pos += keyword.len();
            true
        } else {
            false
        }
    }

    fn number(&mut self) -> Token {
        let start_pos = self.pos;
        while self.pos < self.input.len() && self.current_char().is_digit(10) {
            self.advance();
        }
        let num_str = &self.input[start_pos..self.pos];
        Token::Number(num_str.parse().unwrap())
    }

    fn identifier(&mut self) -> Token {
        let start_pos = self.pos;
        while self.pos < self.input.len() && (self.current_char().is_alphanumeric() || self.current_char() == '_') {
            self.advance();
        }
        let id_str = &self.input[start_pos..self.pos];
        Token::Identifier(id_str.to_string())
    }
}
