mod errors;
pub mod interpret;
pub mod lex;
pub mod output;
pub mod parse;
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
    let mut interpreter = interpret::Interpreter::new(ast);
    let output = interpreter.interpret();
    let output = serde_json::to_string_pretty(&output).unwrap();
    fs::write("./output.json", format!("{output}"))?;
    println!("{output:?}");
    Ok(())
}
