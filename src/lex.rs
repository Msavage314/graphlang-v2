use core::iter::Peekable;
use std::vec::IntoIter;

use strum::EnumDiscriminants;

/// Valid identifier characters
macro_rules! characters {
    () => {
        'a'..='z' | 'A'..='Z' | '$'
    };
}

/// List of possible tokens
///
/// Enum discriminants automatically generates tokenkind from this
#[derive(Debug, Clone, PartialEq, Eq, EnumDiscriminants)]
#[strum_discriminants(name(TokenKind))]
pub enum Token {
    Int(isize),
    Float(String),
    Identifier(String),

    Assign,

    Plus,
    Minus,
    Mul,
    Div,
    Pow,
    Bar,

    Equals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    Fn,
    Namespace,
    Macro,
    /// Different to identifier, as contains !
    MacroCall(String),
    If,
    Else,

    Hash,
    Comma,
    Dot,
    Bang,

    Comment(String),

    Eof,
    At,
    Str(String),
}

impl Token {
    pub fn kind(&self) -> TokenKind {
        TokenKind::from(self)
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
        Self {
            text: text
                .to_string()
                .chars()
                .collect::<Vec<char>>()
                .into_iter()
                .peekable(),
            position: 0,
        }
    }
    fn read_char(&mut self) -> Option<char> {
        self.position += 1;
        self.text.next()
    }
    fn peek_char(&mut self) -> Option<&char> {
        self.text.peek()
    }
    fn parse_int(&mut self, c: char) -> Token {
        let mut s = String::new();
        s.push(c);

        while let Some('0'..='9') = self.peek_char() {
            s.push(self.read_char().unwrap());
        }
        if let Some('.') = self.peek_char() {
            self.read_char();
            return self.lex_decimal(s);
        }

        Token::Int(s.parse::<isize>().expect("Failed to parse integer"))
    }
    fn lex_decimal(&mut self, mut s: String) -> Token {
        if s.is_empty() {
            s.push('0');
        }
        s.push('.');
        let mut digits = 0;
        while let Some('0'..='9') = self.peek_char() {
            s.push(self.read_char().unwrap());
            digits += 1;
        }
        if digits == 0 {
            panic!("Expected digits after decimal point in '{s}'")
        }
        Token::Float(s)
    }
    fn lex_identifier(&mut self, c: char) -> Token {
        let mut s = String::new();
        s.push(c);

        while let Some(characters!()) = self.peek_char() {
            s.push(self.read_char().unwrap());
        }
        if let Some('!') = self.peek_char() {
            self.read_char();
            s.push('!');
            return Token::MacroCall(s);
        }
        match s.as_str() {
            "fn" => Token::Fn,
            "namespace" => Token::Namespace,
            "macro" => Token::Macro,
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Identifier(s),
        }
    }
    fn lex_comment(&mut self) -> Token {
        let mut string = String::new();
        while let Some(c) = self.peek_char() {
            if *c == '\n' {
                break;
            }
            string.push(self.read_char().unwrap());
        }
        Token::Comment(string)
    }
    fn lex_string(&mut self) -> Token {
        let mut s = String::new();
        loop {
            match self.read_char() {
                Some('"') => break,
                Some(c) => s.push(c),
                None => panic!("Unterminated strin literal"),
            }
        }
        Token::Str(s)
    }
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(c) = self.read_char() {
            let token = match c {
                '0'..='9' => self.parse_int(c),
                characters!() => self.lex_identifier(c),
                '#' => self.lex_comment(),
                '=' => match self.peek_char()? {
                    '=' => Token::Equals,
                    _ => Token::Assign,
                },
                '>' => match self.peek_char()? {
                    '=' => Token::GreaterThanOrEqual,
                    _ => Token::GreaterThan,
                },
                '<' => match self.peek_char()? {
                    '=' => Token::LessThanOrEqual,
                    _ => Token::LessThan,
                },
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Mul,
                '/' => Token::Div,
                '^' => Token::Pow,
                '(' => Token::LParen,
                ')' => Token::RParen,
                ',' => Token::Comma,
                '|' => Token::Bar,
                '[' => Token::LBracket,
                ']' => Token::RBracket,
                '{' => Token::LBrace,
                '}' => Token::RBrace,
                '.' => Token::Dot,
                '!' => Token::Bang,
                '@' => Token::At,
                '"' => self.lex_string(),
                _ => panic!("Unknown char {c}"),
            };
            return Some(token);
        }

        None
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
        self.next_token()
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
