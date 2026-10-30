use core::fmt::Display;

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
    Int(isize),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
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
