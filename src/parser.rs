use std::fs;
use logos::{Logos, Span};

use crate::ast::Program;
use crate::grammar::ProgramParser;
use crate::lex;
use crate::lex::Token;

pub fn parse(filename: String) -> Result<Program, String>{
    let file_str = fs::read_to_string(filename).expect("Unable to parse file");
    let lexer = lex::Token::lexer_with_extras(&file_str, ());
    let stream = lexer.spanned().map(|(t, y): (Token, Span)| (y.start, t, y.end));
    ProgramParser::new().parse(stream).map_err(|e| format!("{}", e))
}