use core::fmt::Display;

use crate::output::latex::ToLatex;

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expression),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Parentheses(Box<Expression>),
    Int(isize),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
}
impl ToLatex for Expression {
    fn to_latex(&self) -> String {
        match self {
            Expression::Parentheses(i) => {
                let left = "\\left(";
                let right = "\\right)";

                format!("{left} {} {right}", i.to_latex())
            }
            Expression::Int(i) => i.to_string(),
            Expression::Identifier(name) => {
                if name.len() == 1 {
                    name.to_string()
                } else {
                    let new = name.split_at_checked(1).unwrap();
                    let string = format!("{}_{{{}}}", new.0, new.1).to_string();
                    string
                }
            }
            Expression::Binary { left, op, right } => {
                let left = left.to_latex();
                let right = right.to_latex();

                let op = op.to_latex();
                format!("{left}{op}{right}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Assign,
}
impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BinaryOp::Add => "+".to_string(),
                BinaryOp::Subtract => "-".to_string(),
                BinaryOp::Multiply => "\\cdot ".to_string(),
                BinaryOp::Divide => "/".to_string(),
                BinaryOp::Assign => "=".to_string(),
            }
        )
    }
}
impl ToLatex for BinaryOp {
    fn to_latex(&self) -> String {
        match self {
            BinaryOp::Add => "+".to_string(),
            BinaryOp::Subtract => "-".to_string(),
            BinaryOp::Multiply => "\\cdot ".to_string(),
            BinaryOp::Divide => "/".to_string(),
            BinaryOp::Assign => "=".to_string(),
        }
    }
}
