use crate::parser::{Expr, Op};
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, expr: Expr) -> Option<i64> {
        match expr {
            Expr::Number(value) => Some(value),
            Expr::Variable(var) => self.variables.get(&var).copied(),
            Expr::BinaryOp(left, op, right) => {
                let left_val = self.interpret(*left)?;
                let right_val = self.interpret(*right)?;
                Some(match op {
                    Op::Plus => left_val + right_val,
                    Op::Minus => left_val - right_val,
                    Op::Into => left_val * right_val, // Updated operator
                    Op::By => left_val / right_val,   // Updated operator
                })
            }
            Expr::Assign(var, expr) => {
                let value = self.interpret(*expr)?;
                self.assign(var, value);
                Some(value)
            }
        }
    }

    pub fn assign(&mut self, var: String, value: i64) {
        self.variables.insert(var, value);
    }
}
