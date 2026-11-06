//! Deals with latex representation

pub const BUILTINS: &[(&str, &str)] = &[
    ("x", "x"),
    ("y", "y"),
    ("t", "t"),
    ("sin", "\\sin"),
    ("cos", "\\cos"),
    ("tan", "\\tan"),
    ("polygon", "\\operatorname{polygon}"),
    ("pi", "\\pi"),
    ("theta", "\\theta"),
];

fn builtin_latex(name: &str) -> Option<&'static str> {
    BUILTINS.iter().find(|(n, _)| *n == name).map(|(_, l)| *l)
}

pub trait ToLatex {
    fn to_latex(&self) -> String;
}
pub fn ident_to_latex(name: &str) -> String {
    if let Some(latex) = builtin_latex(name) {
        return latex.to_string();
    }
    let name = name.replace('.', "");
    if name.len() == 1 {
        name.to_string()
    } else {
        let (first, rest) = name.split_at(1);
        format!("{first}_{{{rest}}}")
    }
}
