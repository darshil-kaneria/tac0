use lalrpop_util::lalrpop_mod;

mod context;
mod ast;
mod lex;
mod parser;
mod args;
mod error;
mod typecheck;

lalrpop_mod!(pub grammar);

fn main() {
    
    let mut ctx = context::Context::new();

    // Parse command line args
    let args_parser = args::parse_args(&mut ctx);
    if args_parser.is_err() {
        println!("{}", args_parser.err().unwrap());
        return;
    }

    println!("Input source file: {:?}\nOutput file: {:?}", ctx.args.input_file, ctx.args.output_file);
    
    // Invoke Parser
    let parse_result = parser::parse(ctx.args.input_file.clone());
    if parse_result.is_err() {
        println!("Error: {}", parse_result.err().unwrap());
        return;
    }

    // If debug is enabled:
    if ctx.args.debug_mode {
        println!("--- Parse Tree ---");
        // Expensive call, may destroy performance but who cares in debug mode
        println!("{:#?}", parse_result.clone().unwrap());

        if ctx.args.debug_ast {
            // Print ast debug stuff
        }

        if ctx.args.debug_ir {
            // Print ir debug stuff
        }

        if ctx.args.debug_asm {
            // Print asm debug stuff
        }
    }

    // Perform typechecking
    let typecheck_result = typecheck::typecheck(&mut ctx, parse_result.unwrap());
    if typecheck_result.is_err() {
        println!("Error: {}", typecheck_result.err().unwrap());
        return;
    }

    if ctx.args.typecheck_only {
        return;
    }

    // Perform various passes (transform and analysis)
    return;
}
