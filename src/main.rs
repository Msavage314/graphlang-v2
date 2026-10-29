mod lex;

use std::{env::args, fs, io::Error};

use crate::lex::Lexer;

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
    let lex = Lexer::new(contents);
    for token in lex {
        println!("{token:?}");
    }

    print!("{}", 1);
    Ok(())
}
