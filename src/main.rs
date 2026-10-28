use core::iter::{self, Peekable, from_fn};
use std::{env::args, fs, io::Error, vec::IntoIter};

#[derive(Debug)]
enum Token {
    Int(isize),
    Identifier(String),

    // Operators
    Assign,
    Plus,
}

/// Iterator over tokens in a given input file
struct Lexer {
    /// Iterator over chars in string. Being Peekable instead of Iterator allows us to look at the next element without consuming it
    text: Peekable<IntoIter<char>>,
    position: usize,
}
impl Lexer {
    fn new(text: String) -> Self {
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
    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        if let Some(c) = self.read_char() {
            let token = match c {
                '0'..='9' => {
                    let mut s = String::new();
                    s.push(c);

                    while let Some('0'..='9') = self.peek_char() {
                        s.push(self.read_char().unwrap());
                    }

                    Token::Int(s.parse::<isize>().expect("Failed to parse integer"))
                }
                'a'..'z' => {
                    let mut s = String::new();
                    s.push(c);
                    while let Some('a'..'z') = self.peek_char() {
                        s.push(self.read_char().unwrap());
                    }

                    Token::Identifier(s)
                }

                '=' => Token::Assign,
                '+' => Token::Plus,

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

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    let contents = fs::read_to_string(
        &args
            .get(1)
            .expect("Pass in a filename with `cargo run -- <filename>`"),
    )?;
    let lex = Lexer::new(contents);
    for token in lex {
        println!("{token:?}");
    }
    Ok(())
}
