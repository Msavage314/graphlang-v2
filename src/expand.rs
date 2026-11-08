//! handles expanding macros

use std::collections::HashMap;

use crate::{
    errors::ExpandError,
    parse::ast::{Condition, Expression, Program, Statement, StyleInfo},
};

/// Expands a program, replacing each macro_call statement with its ast
pub fn expand(program: &mut Program) -> Result<(), ExpandError> {
    let mut macros: HashMap<String, (Vec<String>, Vec<Statement>)> = HashMap::new();

    let mut rest = Vec::new();
    for s in program.statements.drain(..) {
        match s {
            Statement::MacroDef { name, params, body } => {
                macros.insert(format!("{name}!"), (params, body));
            }
            other => rest.push(other),
        }
    }
    program.statements = expand_stmts(rest, &macros, 0)?;

    Ok(())
}

fn expand_stmts(
    stmts: Vec<Statement>,
    macros: &HashMap<String, (Vec<String>, Vec<Statement>)>,
    depth: usize,
) -> Result<Vec<Statement>, ExpandError> {
    if depth > 32 {
        return Err(ExpandError::RecursionLimit);
    }
    // vec of statements
    let mut out: Vec<Statement> = Vec::new();
    for s in stmts {
        match s {
            Statement::MacroCall { name, args } => {
                let (params, body) = macros.get(&name).ok_or(ExpandError::UnknownMacro)?;
                if params.len() != args.len() {
                    return Err(ExpandError::WrongArgCount);
                }

                let bindings: HashMap<&str, &Expression> =
                    params.iter().map(|p| p.as_str()).zip(args.iter()).collect();

                let body = body
                    .iter()
                    .map(|s| subst_stmt(s, &bindings))
                    .collect::<Result<Vec<_>, _>>()?;
                // expand again, as it may contain its own macro calls
                out.extend(expand_stmts(body, macros, depth + 1)?);
            }
            Statement::Namespace { name, stmts } => out.push(Statement::Namespace {
                name,
                stmts: expand_stmts(stmts, macros, depth)?,
            }),
            Statement::Annotated { style, stmt } => {
                let mut expanded = expand_stmts(vec![*stmt], macros, depth)?.into_iter();
                if let Some(first) = expanded.next() {
                    out.push(Statement::Annotated {
                        style,
                        stmt: Box::new(first),
                    });
                }
                out.extend(expanded);
            }

            other => out.push(other),
        }
    }
    Ok(out)
}
fn subst_name(name: &str, bindings: &HashMap<&str, &Expression>) -> Result<String, ExpandError> {
    let (head, tail) = match name.split_once('.') {
        Some((h, t)) => (h, Some(t)),
        None => (name, None),
    };
    let head = match bindings.get(head) {
        Some(Expression::Identifier(n)) => n.clone(),
        Some(_) => return Err(ExpandError::ExpansionError),
        None => head.to_string(),
    };
    Ok(match tail {
        Some(t) => format!("{head}.{t}"),
        None => head,
    })
}
fn subst_expr(
    expr: &Expression,
    bindings: &HashMap<&str, &Expression>,
) -> Result<Expression, ExpandError> {
    Ok(match expr {
        // replace base identifiers with the expression. But wrap in parenthesis so $w * 2 with w = a+b keeps its meaning
        Expression::Identifier(name) => match bindings.get(name.as_str()) {
            Some(arg @ Expression::Binary { .. }) => {
                Expression::Parentheses(Box::new((*arg).clone()))
            }
            Some(arg) => (*arg).clone(),
            // Pass things like $w.a on
            None => Expression::Identifier(subst_name(name, bindings)?),
        },
        Expression::Abs(inner) => Expression::Abs(Box::new(subst_expr(inner, bindings)?)),
        Expression::Parentheses(inner) => {
            Expression::Parentheses(Box::new(subst_expr(inner, bindings)?))
        }
        Expression::Call { name, args } => Expression::Call {
            name: subst_name(name, bindings)?,
            args: args
                .iter()
                .map(|a| subst_expr(a, bindings))
                .collect::<Result<_, _>>()?,
        },
        Expression::Binary { left, op, right } => Expression::Binary {
            left: Box::new(subst_expr(left, bindings)?),
            op: op.clone(),
            right: Box::new(subst_expr(right, bindings)?),
        },
        Expression::Float(n) => Expression::Float(n.clone()),
        Expression::Int(n) => Expression::Int(*n),
        Expression::Index { target, index } => Expression::Index {
            target: Box::new(subst_expr(target, bindings)?),
            index: index
                .iter()
                .map(|a| subst_expr(a, bindings))
                .collect::<Result<_, _>>()?,
        },
        Expression::List(items) => Expression::List(
            items
                .iter()
                .map(|a| subst_expr(a, bindings))
                .collect::<Result<_, _>>()?,
        ),
        Expression::Point(x, y) => Expression::Point(
            Box::new(subst_expr(x, bindings)?),
            Box::new(subst_expr(y, bindings)?),
        ),

        Expression::If {
            condition,
            block,
            else_block,
        } => Expression::If {
            condition: subst_condititon(condition, bindings)?,
            block: Box::new(subst_expr(block, bindings)?),
            else_block: match else_block {
                None => None,
                Some(expr) => Some(Box::new(subst_expr(expr, bindings)?)),
            },
        },
        Expression::Str(s) => Expression::Str(s.clone()),
    })
}
fn subst_stmt(
    s: &Statement,
    bindings: &HashMap<&str, &Expression>,
) -> Result<Statement, ExpandError> {
    Ok(match s {
        Statement::Assignment { name, value } => Statement::Assignment {
            name: subst_name(name, bindings)?,
            value: subst_expr(value, bindings)?,
        },
        Statement::Expr(expr) => Statement::Expr(subst_expr(expr, bindings)?),
        Statement::FunctionDef { name, params, body } => Statement::FunctionDef {
            name: subst_name(name, bindings)?,
            params: params.clone(),
            body: subst_expr(body, bindings)?,
        },
        Statement::Namespace { name, stmts } => Statement::Namespace {
            name: subst_name(name, bindings)?,
            stmts: stmts
                .iter()
                .map(|s| subst_stmt(s, bindings))
                .collect::<Result<_, _>>()?,
        },
        Statement::MacroCall { name, args } => Statement::MacroCall {
            name: name.clone(),
            args: args
                .iter()
                .map(|a| subst_expr(a, bindings))
                .collect::<Result<_, _>>()?,
        },
        Statement::Annotated { style, stmt: body } => Statement::Annotated {
            style: subst_style(style, bindings)?,
            stmt: Box::new(subst_stmt(body, bindings)?),
        },

        other => other.clone(),
    })
}
fn subst_condititon(
    c: &Condition,
    bindings: &HashMap<&str, &Expression>,
) -> Result<Condition, ExpandError> {
    Ok(Condition {
        left: Box::new(subst_expr(&c.left, bindings)?),
        right: Box::new(subst_expr(&c.right, bindings)?),
        op: c.op.clone(),
    })
}
fn subst_style(
    style: &StyleInfo,
    bindings: &HashMap<&str, &Expression>,
) -> Result<StyleInfo, ExpandError> {
    Ok(match style {
        StyleInfo::Color(e) => StyleInfo::Color(subst_expr(e, bindings)?),
        StyleInfo::LineStyle(l) => StyleInfo::LineStyle(subst_expr(l, bindings)?),
        StyleInfo::Hidden => StyleInfo::Hidden,
        StyleInfo::Label(l) => StyleInfo::Label(subst_expr(l, bindings)?),
    })
}
