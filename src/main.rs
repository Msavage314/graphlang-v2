mod lex;
mod parse;
use std::{env::args, fs, io::Error};

use crate::{lex::Lexer, parse::Parser};

const honey_jar: &'static str = "
┌\x1b[4m───────\x1b[0m┐
╭───────╮
│#######│
│ HONEY │
│#######│
╰───────╯
";

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    let contents = fs::read_to_string(
        &args
            .get(1)
            .expect("Pass in a filename with `cargo run -- <filename>`"),
    )?;
    let mut parse = Parser::from_string(contents);
    let ast = parse.parse();
    println!("{ast:?}");
    Ok(())
}
