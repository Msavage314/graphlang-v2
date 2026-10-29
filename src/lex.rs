use core::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug)]
pub enum Token {
    Int(isize),
    Identifier(String),

    // Operators
    Assign,
    Plus,
    Minus,
    Mul,
    Div,
}

/// Iterator over tokens in a given input file
pub struct Lexer {
    /// Iterator over chars in string. Being Peekable instead of Iterator allows us to look at the next element without consuming it
    text: Peekable<IntoIter<char>>,
    position: usize,
}
impl Lexer {
    pub fn new(text: String) -> Self {
        return Self {
            text: text.chars().collect::<Vec<char>>().into_iter().peekable(),
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

        Token::Int(s.parse::<isize>().expect("Failed to parse integer"))
    }
    fn parse_identifier(&mut self, c: char) -> Token {
        let mut s = String::new();
        s.push(c);

        while let Some('a'..'z') = self.peek_char() {
            s.push(self.read_char().unwrap());
        }

        Token::Identifier(s)
    }
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(c) = self.read_char() {
            let token = match c {
                '0'..='9' => self.parse_int(c),
                'a'..'z' => self.parse_identifier(c),

                '=' => Token::Assign,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Mul,
                '/' => Token::Div,
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
