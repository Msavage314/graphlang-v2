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
    Namespace {
        name: String,
        stmts: Vec<Statement>,
    },
    MacroDef {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
    MacroCall {
        name: String,
        args: Vec<Expression>,
    },
    Annotated {
        style: StyleInfo,
        stmt: Box<Statement>,
    },
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
            Statement::Annotated { stmt: body, .. } => body.to_latex(),
            // TODO Potentially change this section somehow?
            Statement::Comment(_) => panic!("Comment doesn't support latex"),
            Statement::Namespace { name: _, stmts: _ } => panic!("Namespace doesn't support latex"),
            // All macros should be expanded by the time we get here anyway
            Statement::MacroDef { .. } => unreachable!(),
            Statement::MacroCall { .. } => unreachable!(),
        }
    }
}

impl Statement {
    /// Returns a reference to the inner of this [`Statement`].
    fn _inner(&self) -> &Statement {
        match self {
            Statement::Annotated { stmt: body, .. } => body._inner(),
            s => s,
        }
    }
    /// Returns a mutable reference to the inner of this [`Statement`].
    pub fn inner_mut(&mut self) -> &mut Statement {
        match self {
            Statement::Annotated { stmt, .. } => stmt.inner_mut(),
            s => s,
        }
    }
}
#[derive(Debug, Clone)]
pub enum Expression {
    Parentheses(Box<Expression>),
    Abs(Box<Expression>),
    Float(String),
    /// An integer. Contains an isize
    Int(isize),
    /// An identifier. Can only contain lowercase characters for now
    Identifier(String),
    /// A bianry operation, such as 1 + 1.
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
    Index {
        target: Box<Expression>,
        index: Vec<Expression>,
    },
    If {
        condition: Condition,
        block: Box<Expression>,
        else_block: Option<Box<Expression>>,
    },
    Str(String),
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
            Expression::Index { target, index } => {
                let index = index
                    .iter()
                    .map(|e| e.to_latex())
                    .collect::<Vec<_>>()
                    .join(",");
                format!("{}\\left[{index}\\right]", target.to_latex())
            }
            Expression::Float(i) => i.clone(),

            Expression::If {
                condition,
                block,
                else_block,
            } => {
                let condition = condition.to_latex();
                let block = block.to_latex();
                if let Some(else_block) = else_block {
                    let else_block = else_block.to_latex();
                    format!("\\left\\{{ {condition}: {block}, {else_block}\\right\\}}",)
                } else {
                    format!("\\left\\{{ {condition}: {block}\\right\\}}",)
                }
            }
            Expression::Str(_) => panic!("String can only be used as an annotation arguments"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub left: Box<Expression>,
    pub op: CompareOp,
    pub right: Box<Expression>,
}
impl ToLatex for Condition {
    fn to_latex(&self) -> String {
        let left = self.left.to_latex();
        let right = self.right.to_latex();
        let op = self.op.to_latex();
        return format!("{left}{op}{right}");
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

#[derive(Debug, Clone)]
pub enum CompareOp {
    Equal,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
}
impl ToLatex for CompareOp {
    fn to_latex(&self) -> String {
        match self {
            CompareOp::Equal => "=".to_string(),
            CompareOp::GreaterThan => ">".to_string(),
            CompareOp::LessThan => "<".to_string(),
            CompareOp::GreaterThanOrEqual => "\\ge".to_string(),
            CompareOp::LessThanOrEqual => "\\le".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum StyleInfo {
    Color(Expression),
    LineStyle(Expression),
}
