use core::iter::Peekable;

use crate::{
    errors::{ParseError, ParserResult},
    lex::{
        Lexer,
        Token::{self},
        TokenKind,
    },
    parse::ast::{BinaryOp, CompareOp, Condition, Expression, Program, Statement},
};

pub struct Parser {
    lex: Peekable<Lexer>,
}

impl Parser {
    pub fn from_lex(lex: Lexer) -> Self {
        Self {
            lex: lex.peekable(),
        }
    }
    pub fn from_string<T>(text: T) -> Self
    where
        T: ToString,
    {
        let lex = Lexer::new(text);
        Self {
            lex: lex.peekable(),
        }
    }

    fn peek(&mut self) -> &Token {
        self.lex.peek().unwrap_or(&Token::Eof)
    }
    fn expect(&mut self, expected: TokenKind) -> Result<Token, ParseError> {
        match self.peek() {
            token if token.kind() == expected => Ok(self.advance()),
            token if token.kind() == TokenKind::Eof => Err(ParseError::UnexpectedEof),
            token => Err(ParseError::UnexpectedToken {
                expected,
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
        false
    }
    fn advance(&mut self) -> Token {
        self.lex.next().unwrap_or(Token::Eof)
    }
    pub fn parse(&mut self) -> ParserResult<Program> {
        self.parse_program()
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
        }
        if self.peek().kind() == TokenKind::Namespace {
            return self.parse_namespace();
        }
        if self.peek().kind() == TokenKind::Comment {
            let Token::Comment(text) = self.advance() else {
                unreachable!()
            };
            return Ok(Statement::Comment(text));
        }
        if self.peek().kind() == TokenKind::Macro {
            return self.parse_macro_def();
        }
        if let Token::MacroCall(_) = self.peek() {
            let Token::MacroCall(name) = self.advance() else {
                unreachable!()
            };
            self.expect(TokenKind::LParen)?;
            let mut args = Vec::new();
            if !self.eat(TokenKind::RParen) {
                loop {
                    args.push(self.parse_expr_stmt()?);
                    if self.eat(TokenKind::Comma) {
                        continue;
                    }
                    self.expect(TokenKind::RParen)?;
                    break;
                }
            }
            return Ok(Statement::MacroCall { name, args });
        }

        let expr = self.parse_expr_stmt()?;

        if self.eat(TokenKind::Assign) {
            let Expression::Identifier(name) = expr else {
                return Err(ParseError::InvalidAssignmentTarget);
            };
            let value = self.parse_expr_stmt()?;
            return Ok(Statement::Assignment { name, value });
        }

        Ok(Statement::Expr(expr))
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
        let mut left = self.parse_postfix()?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                Token::Mul => BinaryOp::Multiply,
                Token::Div => BinaryOp::Divide,
                Token::Pow => BinaryOp::Pow,
                _ => break,
            };
            // Skip over the operator now we know there is one
            self.advance();
            let right = self.parse_postfix()?;
            left = Expression::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> ParserResult<Expression> {
        match self.advance() {
            Token::Int(value) => Ok(Expression::Int(value)),
            Token::Float(value) => Ok(Expression::Float(value)),
            Token::Identifier(first) => {
                let ident = if self.eat(TokenKind::Dot) {
                    let second = self.expect_identifier()?;
                    format!("{first}.{second}")
                } else {
                    first
                };
                if self.eat(TokenKind::LParen) {
                    let mut args = Vec::new();
                    if !self.eat(TokenKind::RParen) {
                        loop {
                            args.push(self.parse_expr_stmt()?);
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
                if self.eat(TokenKind::Comma) {
                    let second = self.parse_expr_stmt()?;
                    self.expect(TokenKind::RParen)?;
                    Ok(Expression::Point(Box::new(expr), Box::new(second)))
                } else {
                    self.expect(TokenKind::RParen)?;
                    Ok(Expression::Parentheses(Box::new(expr)))
                }
            }
            Token::Bar => {
                let expr = self.parse_expr_stmt()?;
                self.expect(TokenKind::Bar)?;
                Ok(Expression::Abs(Box::new(expr)))
            }
            Token::LBracket => {
                let items = self.parse_list_items()?;
                Ok(Expression::List(items))
            }
            Token::Eof => Err(ParseError::UnexpectedEof),
            Token::If => {
                let condition = self.parse_condition()?;
                self.expect(TokenKind::LBrace)?;
                let block = self.parse_expr_stmt()?;
                self.expect(TokenKind::RBrace)?;
                let else_block = if self.eat(TokenKind::Else) {
                    self.expect(TokenKind::LBrace)?;
                    let temp = Some(Box::new(self.parse_expr_stmt()?));
                    self.expect(TokenKind::RBrace)?;
                    temp
                } else {
                    None
                };

                Ok(Expression::If {
                    condition,
                    block: Box::new(block),
                    else_block,
                })
            }
            token => Err(ParseError::ExpectedExpression {
                found: token.kind(),
            }),
        }
    }
    fn parse_list_items(&mut self) -> ParserResult<Vec<Expression>> {
        let mut items = Vec::new();
        if !self.eat(TokenKind::RBracket) {
            loop {
                items.push(self.parse_expr_stmt()?);
                if self.eat(TokenKind::Comma) {
                    continue;
                }
                self.expect(TokenKind::RBracket)?;
                break;
            }
        }
        Ok(items)
    }
    fn parse_postfix(&mut self) -> ParserResult<Expression> {
        let mut expr = self.parse_primary()?;
        while self.eat(TokenKind::LBracket) {
            let index = self.parse_list_items()?;
            expr = Expression::Index {
                target: Box::new(expr),
                index,
            };
        }
        Ok(expr)
    }

    fn parse_namespace(&mut self) -> ParserResult<Statement> {
        self.expect(TokenKind::Namespace)?;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LBrace)?;

        let mut stmts = Vec::new();
        while self.peek().kind() != TokenKind::RBrace {
            if self.peek().kind() == TokenKind::Eof {
                return Err(ParseError::UnexpectedEof);
            }
            stmts.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;

        Ok(Statement::Namespace { name, stmts })
    }
    fn parse_function(&mut self) -> ParserResult<Statement> {
        self.expect(TokenKind::Fn)?;
        let name = self.expect_identifier()?;

        self.expect(TokenKind::LParen)?;
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
    fn parse_macro_def(&mut self) -> ParserResult<Statement> {
        self.expect(TokenKind::Macro)?;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LParen)?;
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
        self.expect(TokenKind::LBrace)?;
        let mut stmts = Vec::new();
        while self.peek().kind() != TokenKind::RBrace {
            if self.peek().kind() == TokenKind::Eof {
                return Err(ParseError::UnexpectedEof);
            }
            stmts.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(Statement::MacroDef {
            name,
            params,
            body: stmts,
        })
    }
    fn parse_condition(&mut self) -> ParserResult<Condition> {
        let left = self.parse_expr_stmt()?;
        let op = match self.peek().kind() {
            TokenKind::Equals => CompareOp::Equal,
            TokenKind::GreaterThan => CompareOp::GreaterThan,
            TokenKind::GreaterThanOrEqual => CompareOp::GreaterThanOrEqual,
            TokenKind::LessThan => CompareOp::LessThan,
            TokenKind::LessThanOrEqual => CompareOp::LessThanOrEqual,
            kind => {
                return Err(ParseError::UnexpectedToken {
                    expected: TokenKind::Equals,
                    found: kind,
                });
            }
        };
        self.advance();
        let right = self.parse_expr_stmt()?;
        Ok(Condition {
            left: Box::new(left),
            op,
            right: Box::new(right),
        })
    }
}
