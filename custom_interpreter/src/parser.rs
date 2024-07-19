use crate::lexer::{Lexer, Token};

#[derive(Debug)]
pub enum Expr {
    Variable(String),
    Number(i64),
    BinaryOp(Box<Expr>, Op, Box<Expr>),
    Assign(String, Box<Expr>),
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: Token::EndOfFile,
        };
        parser.next_token(); // Initialize the first token
        parser
    }

    pub fn parse(&mut self) -> Option<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> Option<Expr> {
        if self.current_token == Token::Assume {
            self.next_token();
            if let Token::Identifier(var) = self.current_token.clone() {
                self.next_token();
                if self.current_token == Token::Eq {
                    self.next_token();
                    let expr = self.expr()?;
                    return Some(Expr::Assign(var, Box::new(expr)));
                }
            }
        }
        self.expr()
    }

    fn expr(&mut self) -> Option<Expr> {
        let mut left = self.factor()?;

        while matches!(self.current_token, Token::Add | Token::Sub) {
            let op = self.current_token.clone();
            self.next_token();
            let right = self.factor()?;
            left = Expr::BinaryOp(Box::new(left), self.op_from_token(op), Box::new(right));
        }

        Some(left)
    }

    fn factor(&mut self) -> Option<Expr> {
        match self.current_token {
            Token::Number(value) => {
                self.next_token();
                Some(Expr::Number(value))
            }
            Token::Identifier(ref id) => {
                let id_clone = id.clone();
                self.next_token();
                Some(Expr::Variable(id_clone))
            }
            _ => None,
        }
    }

    fn op_from_token(&self, token: Token) -> Op {
        match token {
            Token::Add => Op::Add,
            Token::Sub => Op::Sub,
            Token::Mul => Op::Mul,
            Token::Div => Op::Div,
            _ => panic!("Invalid operator"),
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}
