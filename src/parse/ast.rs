use crate::output::latex::{ToLatex, ident_to_latex};

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expr(Expression),
    FunctionDef {
        name: String,
        params: Vec<String>,
        body: Expression,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    Comment(String),
}
impl ToLatex for Statement {
    fn to_latex(&self) -> String {
        match self {
            Statement::Expr(e) => e.to_latex(),
            Statement::FunctionDef { name, params, body } => {
                let params = params
                    .iter()
                    .map(|p| ident_to_latex(p))
                    .collect::<Vec<_>>()
                    .join(",");
                format!(
                    "{}\\left({params}\\right)={}",
                    ident_to_latex(name),
                    body.to_latex()
                )
            }
            Statement::Assignment { name, value: body } => {
                format!("{} = {}", ident_to_latex(name), body.to_latex())
            }
            // TODO Potentially change this section somehow?
            Statement::Comment(_) => panic!("Comment doesn't support latex"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Parentheses(Box<Expression>),
    Abs(Box<Expression>),
    Int(isize),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Call {
        name: String,
        args: Vec<Expression>,
    },
    Point(Box<Expression>, Box<Expression>),
    List(Vec<Expression>),
}

impl ToLatex for Expression {
    fn to_latex(&self) -> String {
        match self {
            Expression::Parentheses(i) => {
                let left = "\\left(";
                let right = "\\right)";

                format!("{left} {} {right}", i.to_latex())
            }
            Expression::Abs(i) => {
                let left = "\\left|";
                let right = "\\right|";

                format!("{left} {} {right}", i.to_latex())
            }
            Expression::Int(i) => i.to_string(),
            Expression::Identifier(name) => ident_to_latex(name),
            Expression::Binary { left, op, right } => {
                let left = left.to_latex();
                let right = right.to_latex();
                let op_latex = op.to_latex();
                match op {
                    BinaryOp::Divide => {
                        format!("\\frac{{{left}}}{{{right}}}")
                    }
                    BinaryOp::Pow => {
                        format!("{{{left}}}^{{{right}}}")
                    }
                    _ => {
                        format!("{left}{op_latex}{right}")
                    }
                }
            }
            Expression::Call { name, args } => {
                let name = ident_to_latex(name);
                let args = args
                    .iter()
                    .map(|a| a.to_latex())
                    .collect::<Vec<_>>()
                    .join(",");

                format!("{name}\\left({args}\\right)")
            }
            Expression::Point(x, y) => {
                format!("\\left({},{}\\right)", x.to_latex(), y.to_latex())
            }
            Expression::List(items) => {
                format!(
                    "\\left[{}\\right]",
                    items
                        .iter()
                        .map(|e| e.to_latex())
                        .collect::<Vec<_>>()
                        .join(",")
                )
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
    Pow,
}

impl ToLatex for BinaryOp {
    fn to_latex(&self) -> String {
        match self {
            BinaryOp::Add => "+".to_string(),
            BinaryOp::Subtract => "-".to_string(),
            BinaryOp::Multiply => "\\cdot ".to_string(),
            BinaryOp::Divide => "/".to_string(),
            BinaryOp::Pow => "^".to_string(),
        }
    }
}
