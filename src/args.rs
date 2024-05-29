use crate::context::Context;
use crate::error;

pub struct CommandlineArgs {
    pub input_file: String,
    pub optimization_level: u8,
    pub disable_opt: Vec<String>,
    pub output_file: String,
    pub linker: String,
    
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
            "-l" => {

                ctx.args.linker = args[i+1].clone();
                i += 1;
            }
            "-d" | "--debug" => ctx.args.debug_mode = true,
            "--debug-ast" => ctx.args.debug_ast = true,
            "--debug-asm" => ctx.args.debug_asm = true,
            "--debug-ir" => {
                ctx.args.debug_ir = true;
                ctx.args.debug_ir_lvl = args[i+1].clone().parse().unwrap();
                i += 1;
            },
            "-o" | "--output-file" => {
                ctx.args.output_file = args[i+1].clone();
            },
            
            _ => {
                if ctx.args.input_file.is_empty() {
                    ctx.args.input_file = args[i].clone();
                    i += 1;
                } else {
                    return Err(error::ArgParseError(format!("Unexpected argument: {}", args[i])));
                }
            }
        }
    }

    Ok(())
}