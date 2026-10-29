trait ASTNode {}

pub struct Program {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    Expr(Expr),
}

pub enum Expr {
    Int,
}
