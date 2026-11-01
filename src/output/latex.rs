//! Deals with latex representation

pub trait ToLatex {
    fn to_latex(&self) -> String;
}
pub fn ident_to_latex(name: &str) -> String {
    if name.len() == 1 {
        name.to_string()
    } else {
        let (first, rest) = name.split_at(1);
        format!("{first}_{{{rest}}}")
    }
}
