use core::iter::Peekable;
use std::vec::IntoIter;

use strum::EnumDiscriminants;

/// List of possible tokens
///
/// Enum discriminants automatically generates tokenkind from this
#[derive(Debug, Clone, PartialEq, Eq, EnumDiscriminants)]
#[strum_discriminants(name(TokenKind))]
pub enum Token {
    Int(isize),
    Identifier(String),
    Assign,
    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    LParen,
    RParen,
    Comma,
    Bar,
    Fn,
    Eof,
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        return TokenKind::from(self);
    }
}

/// Iterator over tokens in a given input file
pub struct Lexer {
    /// Iterator over chars in string. Being Peekable instead of Iterator allows us to look at the next element without consuming it
    text: Peekable<IntoIter<char>>,
    position: usize,
}
impl Lexer {
    pub fn new<T>(text: T) -> Self
    where
        T: ToString,
    {
        return Self {
            text: text
                .to_string()
                .chars()
                .collect::<Vec<char>>()
                .into_iter()
                .peekable(),
            position: 0,
        };
    }
    fn read_char(&mut self) -> Option<char> {
        self.position += 1;
        return self.text.next();
    }
    fn peek_char(&mut self) -> Option<&char> {
        return self.text.peek();
    }
    fn parse_int(&mut self, c: char) -> Token {
        let mut s = String::new();
        s.push(c);

        while let Some('0'..='9') = self.peek_char() {
            s.push(self.read_char().unwrap());
        }

        return Token::Int(s.parse::<isize>().expect("Failed to parse integer"));
    }
    fn parse_identifier(&mut self, c: char) -> Token {
        let mut s = String::new();
        s.push(c);

        while let Some('a'..='z') = self.peek_char() {
            s.push(self.read_char().unwrap());
        }
        return match s.as_str() {
            "fn" => Token::Fn,
            _ => Token::Identifier(s),
        };
    }
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(c) = self.read_char() {
            let token = match c {
                '0'..='9' => self.parse_int(c),
                'a'..='z' => self.parse_identifier(c),

                '=' => Token::Assign,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Mul,
                '/' => Token::Div,
                '^' => Token::Pow,
                '(' => Token::LParen,
                ')' => Token::RParen,
                ',' => Token::Comma,
                '|' => Token::Bar,

                _ => panic!("Unknown char {c}"),
            };
            return Some(token);
        }

        return None;
    }
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.read_char();
            } else {
                return;
            }
        }
    }
}
impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        return self.next_token();
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::lex::{Lexer, Token};

    #[test]
    fn test_lex() {
        let lex = Lexer::new("x = 1\ny = 1\nx = x + y\nresult = x + 1 * 2 - 3 / 4".to_string());

        let intended = vec![
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Int(1),
            Token::Identifier("y".to_string()),
            Token::Assign,
            Token::Int(1),
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Identifier("y".to_string()),
            Token::Identifier("result".to_string()),
            Token::Assign,
            Token::Identifier("x".to_string()),
            Token::Plus,
            Token::Int(1),
            Token::Mul,
            Token::Int(2),
            Token::Minus,
            Token::Int(3),
            Token::Div,
            Token::Int(4),
        ];

        for (index, token) in lex.enumerate() {
            let Some(wanted) = intended.get(index) else {
                panic!("Expected end of token stream, got {token:?}")
            };
            assert_eq!(
                token, *wanted,
                "expected token {wanted:?}, found token {token:?}"
            );
        }
    }
}
