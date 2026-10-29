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
pub struct ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error")?;
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
            token if token.kind() == TokenKind::Eof => Err(ParseError {}),
            _ => Err(ParseError {}),
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
    pub fn parse(&mut self) -> ParserResult<Program> {
        self.parse_program()
    }
    fn parse_program(&mut self) -> ParserResult<Program> {
        let mut statements = Vec::new();
        loop {
            if let Ok(stmt) = self.parse_stmt() {
                statements.push(stmt);
            } else {
                break;
            }
        }
        let p = Program { statements };
        return Ok(p);
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

    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.advance() {
            Token::Int(value) => Ok(Expr::Int(value)),
            Token::Eof => Err(ParseError {}),
            _ => Err(ParseError {}),
        }
    }
}
