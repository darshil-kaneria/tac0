// src/main.rs

mod frontend;
mod middle;

use frontend::ast::*;
use frontend::parser;
use std::fs;

fn main() {
    // Read the source code from a file (or hardcode a string for now).
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let src = fs::read_to_string(filename).expect("Unable to read file");

    // Parse the input into an AST.
    match parser::ProgramParser::new().parse(&src) {
        Ok(ast) => {
            println!("Parsed AST: {:#?}", ast);
            // In Phase 1, you can then convert the AST to IR.
            // For now, you might simply print or further process the AST.
        }
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
        }
    }
}
