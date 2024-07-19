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
    Plus,
    Minus,
    Into,
    By,
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
            self.next_token(); // Move past 'Assume'
    
            // Ensure the next token is an identifier
            if let Token::Identifier(ref var) = self.current_token {
                let var_name = var.clone();
                self.next_token(); // Move past the identifier
    
                // Ensure the next token is '='
                if self.current_token == Token::Eq {
                    self.next_token(); // Move past '='
    
                    // Parse the expression after '='
                    if let Some(expr) = self.expr() {
                        return Some(Expr::Assign(var_name, Box::new(expr)));
                    }
                }
            }
        }
        // If not an assignment, parse it as an expression
        self.expr()
    }
    

    fn expr(&mut self) -> Option<Expr> {
        self.add_sub()
    }

    fn add_sub(&mut self) -> Option<Expr> {
        let mut left = self.mul_div()?;

        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = self.current_token.clone();
            self.next_token();
            let right = self.mul_div()?;
            left = Expr::BinaryOp(Box::new(left), self.op_from_token(op), Box::new(right));
        }

        Some(left)
    }

    // fn mul_div(&mut self) -> Option<Expr> {
    //     let mut left = self.factor()?;

    //     while matches!(self.current_token, Token::Into | Token::By) {
    //         let op = self.current_token.clone();
    //         self.next_token();
    //         let right = self.factor()?;
    //         left = Expr::BinaryOp(Box::new(left), self.op_from_token(op), Box::new(right));
    //     }

    //     Some(left)
    // }

    fn mul_div(&mut self) -> Option<Expr> {
        let mut left = self.factor()?;
    
        while matches!(self.current_token, Token::Into | Token::By) {
            let op = self.current_token.clone();
            self.next_token();
            let right = self.factor()?;
    
            // Check for division by zero
            if let Token::By = op {
                if let Expr::Number(0) = right {
                    println!("Error: Division by zero is not possible.");
                    return None;
                }
            }
    
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
            Token::LParen => {
                self.next_token();
                let expr = self.expr()?;
                if self.current_token == Token::RParen {
                    self.next_token();
                    Some(expr)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn op_from_token(&self, token: Token) -> Op {
        match token {
            Token::Plus => Op::Plus,
            Token::Minus => Op::Minus,
            Token::Into => Op::Into,
            Token::By => Op::By,
            _ => panic!("Invalid operator"),
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
    }
}
