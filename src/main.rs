use lalrpop_util::lalrpop_mod;

mod context;
mod ast;
mod lex;
mod parser;
mod args;
mod error;

lalrpop_mod!(pub grammar);

fn main() {
    
    let mut ctx = context::Context::new();

    // Parse command line args
    let args_parser = args::parse_args(&mut ctx);
    if args_parser.is_err() {
        println!("{}", args_parser.err().unwrap());
        return;
    }

    println!("{:?}", ctx.args.input_file);
    
    // Invoke Parser
    let parse_result = parser::parse(ctx.args.input_file);
    if parse_result.is_err() {
        println!("Error: {}", parse_result.err().unwrap());
        return;
    }

}
