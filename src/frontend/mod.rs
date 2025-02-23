use lalrpop_util::lalrpop_mod;

pub mod ast;
pub mod lexer;

lalrpop_mod!(pub parser, "/frontend/parser.rs");