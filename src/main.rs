mod frontend;
mod middle;

use frontend::ast::*;
use frontend::parser;
use middle::ir::*;
use middle::irgen;
use middle::pass_manager::*;
use middle::print_ir;
use middle::ssa;
use std::convert;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source file>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];
    let src = fs::read_to_string(filename).expect("Unable to read file");
    let ast = parser::ProgramParser::new().parse(&src).expect("Parse error");

    // To lower ast to IR, we have an IR generator.
    let mut irgen = irgen::IRGenerator::new();
    let ir = irgen.lower_program(&ast);


    let mut pass_manager = PassManager::new();

    // Start adding all the passes in order
    pass_manager.add_pass(print_ir::print_ir);
    pass_manager.add_pass(ssa::convert_to_ssa);

    pass_manager.run(ir);

}
