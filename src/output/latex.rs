//! Deals with latex representation

pub trait ToLatex {
    fn to_latex(&self) -> String;
}
