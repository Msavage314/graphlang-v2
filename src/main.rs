pub mod errors;
pub mod interpret;
pub mod lex;
pub mod output;
pub mod parse;
pub mod semantic;
use core::error::Error;
use std::{env::args, fs};

use crate::{parse::parser::Parser, semantic::resolve};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    let contents = fs::read_to_string(
        args.get(1)
            .expect("Pass in a filename with `cargo run -- <filename>`"),
    )
    .expect("Failed to read args");
    let mut parse = Parser::from_string(contents);
    let mut ast = parse.parse()?;
    resolve(&mut ast);
    let mut interpreter = interpret::Interpreter::new(ast);
    let output = interpreter.interpret()?;
    let output = serde_json::to_string_pretty(&output).expect("Failed to parse to JSON");
    fs::write("./output.json", &output).expect("Failed to write to file");
    //println!("{output}");
    clipboard_anywhere::set_clipboard(&output).expect("Failed to copy to clipboard");
    println!("Done");
    Ok(())
}
