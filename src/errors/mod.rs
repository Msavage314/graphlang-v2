use crate::lex::TokenKind;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
    },
    UnexpectedEof,
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEof => write!(f, "Parser error - Unexpected end of file",)?,
            Self::UnexpectedToken { expected, found } => write!(
                f,
                "Parser error - Expected token {expected:?}, but found token {found:?}"
            )?,
        };
        Ok(())
    }
}
impl core::error::Error for ParseError {}

pub type ParserResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum InterpretError {
    InvalidSyntax,
}
impl core::fmt::Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::InvalidSyntax => write!(f, "Interpret Error - invalid syntax")?,
        }
        Ok(())
    }
}
impl core::error::Error for InterpretError {}

pub type InterpretResult = Result<(), InterpretError>;
