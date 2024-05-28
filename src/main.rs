use lalrpop_util::lalrpop_mod;

mod ast;
mod lex;
mod parser;

lalrpop_mod!(pub grammar);

fn main() {
    parser::parse("sample.c".to_string());
}
