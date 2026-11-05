use std::collections::HashSet;

use crate::parse::ast::{Expression, Program, Statement};

const BUILTINS: &[&str] = &["x", "y", "t"];

fn is_builtin(name: &str) -> bool {
    BUILTINS.contains(&name)
}
pub fn resolve(program: &mut Program) {
    for stmt in program.statements.iter_mut() {
        if let Statement::Namespace { name, stmts } = stmt {
            resolve_namespace(name, stmts);
        }
    }
}
fn resolve_namespace(ns: &str, stmts: &mut [Statement]) {
    // Pass 1: names defined directly in this namespace
    let mut defined = HashSet::new();
    for s in stmts.iter() {
        match s {
            Statement::Assignment { name, .. } | Statement::FunctionDef { name, .. }
                if !is_builtin(name) =>
            {
                defined.insert(name.clone());
            }
            _ => {}
        }
    }
    for s in stmts.iter_mut() {
        match s {
            Statement::Assignment { name, value } => {
                resolve_expr(value, ns, &defined, &HashSet::new());
                if !is_builtin(name) {
                    *name = format!("{ns}.{name}");
                }
            }
            Statement::FunctionDef { name, params, body } => {
                let locals: HashSet<String> = params.iter().cloned().collect();
                resolve_expr(body, ns, &defined, &locals);
                *name = format!("{ns}.{name}")
            }
            Statement::Expr(e) => resolve_expr(e, ns, &defined, &HashSet::new()),
            _ => {}
        }
    }
}
// replaces an expression with its scope correct expression.
fn resolve_expr(
    e: &mut Expression,
    namespace: &str,
    defined: &HashSet<String>,
    locals: &HashSet<String>,
) {
    match e {
        Expression::Identifier(name) => {
            if !locals.contains(name) && defined.contains(name) {
                *name = format!("{namespace}.{name}")
            }
        }
        Expression::Call { name, args } => {
            if defined.contains(name) {
                *name = format!("{namespace}.{name}");
            }
            for a in args {
                resolve_expr(a, namespace, defined, locals);
            }
        }
        Expression::Abs(expr) => {
            resolve_expr(expr, namespace, defined, locals);
        }
        Expression::Binary { left, right, .. } => {
            resolve_expr(left, namespace, defined, locals);
            resolve_expr(right, namespace, defined, locals);
        }
        Expression::Parentheses(expr) => {
            resolve_expr(expr, namespace, defined, locals);
        }
        Expression::Index { target, index } => {
            resolve_expr(target, namespace, defined, locals);
            for i in index {
                resolve_expr(i, namespace, defined, locals);
            }
        }
        Expression::Int(_) => {}
        Expression::List(items) => {
            for i in items {
                resolve_expr(i, namespace, defined, locals);
            }
        }
        Expression::Point(x, y) => {
            resolve_expr(x, namespace, defined, locals);
            resolve_expr(y, namespace, defined, locals);
        }
    }
}
