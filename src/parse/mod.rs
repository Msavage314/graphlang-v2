use core::{
    error::Error,
    fmt::{self},
    iter::Peekable,
};
mod ast;
use crate::{
    lex::{Lexer, Token, TokenKind},
    parse::ast::{BinaryOp, Expr, Program, Statement},
};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
    },
    UnexpectedEof,
}

impl fmt::Display for ParseError {
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
impl Error for ParseError {}

pub type ParserResult<T> = Result<T, ParseError>;

pub struct Parser {
    lex: Peekable<Lexer>,
}

impl Parser {
    pub fn from_lex(lex: Lexer) -> Self {
        return Self {
            lex: lex.peekable(),
        };
    }
    pub fn from_string<T>(text: T) -> Self
    where
        T: ToString,
    {
        let lex = Lexer::new(text);
        return Self {
            lex: lex.peekable(),
        };
    }

    fn peek(&mut self) -> &Token {
        return self.lex.peek().unwrap_or(&Token::Eof);
    }
    fn expect(&mut self, expected: TokenKind) -> Result<Token, ParseError> {
        match self.peek() {
            token if token.kind() == expected => Ok(self.advance()),
            token if token.kind() == TokenKind::Eof => Err(ParseError::UnexpectedEof),
            token => Err(ParseError::UnexpectedToken {
                expected: expected,
                found: token.kind(),
            }),
        }
    }
    fn eat(&mut self, token_type: TokenKind) -> bool {
        let next = self.peek();
        if next.kind() == token_type {
            self.advance();
            return true;
        }
        return false;
    }
    fn advance(&mut self) -> Token {
        self.lex.next().unwrap_or(Token::Eof)
    }
    pub fn parse(&mut self) -> Program {
        match self.parse_program() {
            Ok(p) => {
                return p;
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
    fn parse_program(&mut self) -> ParserResult<Program> {
        let mut statements = Vec::new();

        while self.peek().kind() != TokenKind::Eof {
            statements.push(self.parse_stmt()?);
        }

        Ok(Program { statements })
    }
    fn parse_stmt(&mut self) -> ParserResult<Statement> {
        match self.peek() {
            _ => Ok(Statement::Expr(self.parse_expr_stmt()?)),
        }
    }
    fn parse_expr_stmt(&mut self) -> ParserResult<Expr> {
        let mut left = self.parse_primary()?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                Token::Mul => BinaryOp::Multiply,
                Token::Div => BinaryOp::Divide,
                _ => break,
            };
            // Skip over the operator now we know there is one
            self.advance();
            let right = self.parse_primary()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        }
        return Ok(left);
    }

    fn parse_primary(&mut self) -> ParserResult<Expr> {
        match self.advance() {
            Token::Int(value) => Ok(Expr::Int(value)),
            Token::Identifier(ident) => Ok(Expr::Identifier(ident)),
            Token::Eof => Err(ParseError::UnexpectedEof),
            token => Err(ParseError::UnexpectedToken {
                expected: TokenKind::Int,
                found: token.kind(),
            }),
        }
    }
}
