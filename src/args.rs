use crate::context::Context;
use crate::error;

pub struct CommandlineArgs {
    pub input_file: String,
    pub optimization_level: u8,
    pub disable_opt: Vec<String>,
    pub output_file: String,
    pub linker: String,
    pub typecheck_only: bool,
    
    // Debug options
    pub debug_mode: bool,
    pub debug_ast: bool,
    pub debug_ir: bool,
    pub debug_ir_lvl: u32,
    pub debug_asm: bool,
}

impl CommandlineArgs {
    pub fn new() -> CommandlineArgs {
        CommandlineArgs {
            input_file: String::new(),
            optimization_level: 0,
            disable_opt: Vec::new(),
            output_file: String::new(),
            linker: String::new(),
            typecheck_only: false,
            debug_mode: false,
            debug_ast: false,
            debug_asm: false,
            debug_ir: false,
            debug_ir_lvl: 0
        }
    }
}

pub fn parse_args(ctx: &mut Context) -> Result<(), error::ArgParseError> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return Err(error::ArgParseError(format!("Usage: taco [--options] [--debug-options] <input>")));
    }

    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {

            "-O1" => ctx.args.optimization_level = 1,
            "-O2" => ctx.args.optimization_level = 2,
            "-O3" => ctx.args.optimization_level = 3,
            "-t" | "--typecheck-only" => ctx.args.typecheck_only = true,
            "-l" => {
                ctx.args.linker = args[i+1].clone();
            }
            "-d" | "--debug" => ctx.args.debug_mode = true,
            "--debug-ast" => ctx.args.debug_ast = true,
            "--debug-asm" => ctx.args.debug_asm = true,
            "--debug-ir" => {
                ctx.args.debug_ir = true;
                ctx.args.debug_ir_lvl = args[i+1].clone().parse().unwrap();
            },
            "-o" | "--output-file" => {
                ctx.args.output_file = args[i+1].clone();
                i += 1;
            },

            "--help" => {
                println!("Usage: taco [--options] [--debug-options] <input>");
                println!("Options:");
                println!("\t-O1\t\tOptimization Level 1");
                println!("\t-O2\t\tOptimization Level 2");
                println!("\t-O3\t\tOptimization Level 3");
                println!("\t-l\t\tLinker");
                println!("\t-o\t\tOutput file");
                println!("\t--help\t\tDisplay this help message");
                println!("\t-d, --debug\tEnable debug mode");
                println!("Debug Options:");
                println!("\t--debug-ast\t\tPrint AST");
                println!("\t--debug-asm\t\tPrint Assembly");
                println!("\t--debug-ir\t\tPrint IR");
                return Err(error::ArgParseError("".to_string()));
            }
            
            _ => {
                if ctx.args.input_file.is_empty() {
                    ctx.args.input_file = args[i].clone();
                } else {
                    return Err(error::ArgParseError(format!("Unexpected argument: {}", args[i])));
                }
            }
        }

        i += 1;

    }

    if ctx.args.output_file.is_empty() {
        ctx.args.output_file = ctx.args.input_file.clone().split(".").collect::<Vec<&str>>()[0].to_string();
    }

    Ok(())
}