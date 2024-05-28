
use std::fs;
use logos::Logos;

use crate::grammar::ProgramParser;
use crate::lex;

pub fn parse(filename: String) {
    let file_str = fs::read_to_string(filename).expect("Unable to parse file");
    let lexer = lex::Token::lexer_with_extras(&file_str, ());
    let stream = lexer.spanned().map(|(token, span)| (token, span.start, span.end));
    let ast = ProgramParser::new().parse(stream);
}