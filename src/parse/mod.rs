use core::{
    error::Error,
    fmt::{self},
    iter::Peekable,
};
pub mod ast;
use crate::{
    errors::{ParseError, ParserResult},
    lex::{Lexer, Token, TokenKind},
    parse::ast::{BinaryOp, Expression, Program, Statement},
};

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
        if self.peek().kind() == TokenKind::Fn {
            return self.parse_function();
        } else {
            return Ok(Statement::Expr(self.parse_expr_stmt()?));
        }
    }
    fn parse_function(&mut self) -> ParserResult<Statement> {
        self.expect(TokenKind::Fn)?;
        let name = self.expect_identifier()?;

        self.expect(TokenKind::LParen);
        let mut params = Vec::new();
        if self.peek().kind() != TokenKind::RParen {
            loop {
                params.push(self.expect_identifier()?);
                if !self.eat(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Assign)?;
        let body = self.parse_expr_stmt()?;
        Ok(Statement::FunctionDef { name, params, body })
    }
    fn expect_identifier(&mut self) -> ParserResult<String> {
        match self.advance() {
            Token::Identifier(name) => Ok(name),
            Token::Eof => Err(ParseError::UnexpectedEof),
            token => Err(ParseError::UnexpectedToken {
                expected: TokenKind::Identifier,
                found: token.kind(),
            }),
        }
    }
    fn parse_expr_stmt(&mut self) -> ParserResult<Expression> {
        let mut left = self.parse_primary()?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                Token::Mul => BinaryOp::Multiply,
                Token::Div => BinaryOp::Divide,
                Token::Assign => BinaryOp::Assign,
                _ => break,
            };
            // Skip over the operator now we know there is one
            self.advance();
            let right = self.parse_primary()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        }
        return Ok(left);
    }

    fn parse_primary(&mut self) -> ParserResult<Expression> {
        match self.advance() {
            Token::Int(value) => Ok(Expression::Int(value)),
            Token::Identifier(ident) => {
                if self.eat(TokenKind::LParen) {
                    let mut args = Vec::new();
                    if !self.eat(TokenKind::RParen) {
                        loop {
                            args.push(self.parse_primary()?);
                            if self.eat(TokenKind::Comma) {
                                continue;
                            } // add a Comma token if you don't have one
                            self.expect(TokenKind::RParen)?;
                            break;
                        }
                    }
                    Ok(Expression::Call { name: ident, args })
                } else {
                    Ok(Expression::Identifier(ident))
                }
            }
            Token::LParen => {
                let expr = self.parse_expr_stmt()?;
                self.expect(TokenKind::RParen)?;
                Ok(Expression::Parentheses(Box::new(expr)))
            }
            Token::Eof => Err(ParseError::UnexpectedEof),
            token => Err(ParseError::UnexpectedToken {
                expected: TokenKind::Int,
                found: token.kind(),
            }),
        }
    }
}
