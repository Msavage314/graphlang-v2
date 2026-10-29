use core::{
    error::Error,
    fmt::{self},
    iter::Peekable,
};
mod ast;
use crate::{
    lex::{Lexer, Token},
    parse::ast::{Expr, Program, Statement},
};

#[derive(Debug)]
struct ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parser error")?;
        Ok(())
    }
}
impl Error for ParserError {}

type ParserResult<T> = Result<T, ParserError>;

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
    fn expect(&mut self, expected: Token) -> Result<Token,ParserError> {
        match  self.lex.next() {
            Some(token) if token == expected => Ok(token),
            Some(token) => Err(ParserError {}),

            None => Err(ParserError {})
        }
    }
    fn eat(&mut self, token_type: Tok)
    fn advance(&mut self) -> Token {
        self.lex.next().unwrap_or(Token::Eof)
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
    fn parse_expr_stmt(&mut self) -> ParserResult<Expr> {}
}
