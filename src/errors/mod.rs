use crate::lex::TokenKind;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
    },
    UnexpectedEof,
    ExpectedExpression {
        found: TokenKind,
    },
    InvalidAssignmentTarget,
}

impl core::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedEof => write!(f, "Parser error - Unexpected end of file",)?,
            ParseError::UnexpectedToken { expected, found } => write!(
                f,
                "Parser error - Expected token {expected:?}, but found token {found:?}"
            )?,
            ParseError::ExpectedExpression { found } => {
                write!(f, "Parser error - Expected expression but found {found:?}")?
            }
            ParseError::InvalidAssignmentTarget => {
                write!(f, "Parser error - InvalidAssignmentTarget")?
            }
        };
        Ok(())
    }
}
impl core::error::Error for ParseError {}

pub type ParserResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub enum InterpretError {
    InvalidSyntax,
    NestedNamespace,
}
impl core::fmt::Display for InterpretError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            InterpretError::InvalidSyntax => write!(f, "Interpret Error - invalid syntax")?,
            InterpretError::NestedNamespace => write!(f, "Interpret Error - Nested Namespace")?,
        }
        Ok(())
    }
}
impl core::error::Error for InterpretError {}

pub type InterpretResult = Result<(), InterpretError>;

#[derive(Debug)]
pub enum ExpandError {
    ExpansionError,
    RecursionLimit,
    UnknownMacro,
    WrongArgCount,
}
impl core::fmt::Display for ExpandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ExpandError::ExpansionError => write!(f, "Expansion Error")?,
            ExpandError::RecursionLimit => write!(f, "Reached macro expansion recursion limit")?,
            ExpandError::UnknownMacro => write!(f, "Unknown macro")?,
            ExpandError::WrongArgCount => write!(f, "Incorrect number of arguments to macro")?,
        }
        Ok(())
    }
}
impl core::error::Error for ExpandError {}
