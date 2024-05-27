use logos::Logos;
use logos::Lexer;
use std::{fmt, num::ParseIntError};
use core::fmt::{Display, Formatter};

fn from_num<'b>(lex: &mut Lexer<'b, Token<'b>>) -> Result<i64, String> {
    let slice = lex.slice();
  
    let res: Result<i64, ParseIntError> = slice.parse();
  
    if res.is_err() {
      return Err(format!("Parsing failed wtih Error {:?}", res.unwrap_err()));
    }
    let out = res.unwrap();
    if out > ((i32::MIN as i64).abs()) {
      // All numbers are positive because - is lexed seperately
      return Err(format!("Number {} is out of bounds", out));
    }
    Ok(out)
  }

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Token<'a> {
    // Operators and other symbols
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("%")]
    Modulo,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("==")]
    DoubleEqual,

    #[token("=")]
    Equal,

    #[token(";")]
    Semicolon,

    #[token(":")]
    Colon,

    #[token("++")]
    PlusPlus,

    #[token("--")]
    MinusMinus,

    #[token("->")]
    Arrow,

    #[token("=>")]
    GreaterThanEqual,

    #[token(">")]
    GreaterThan,

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanEqual,

    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token("&&")]
    LogicalAnd,

    #[token("||")]
    LogicalOr,

    #[token("&")]
    BitwiseAnd,

    #[token("|")]
    BitwiseOr,

    #[token("*=")]
    StarEqual,

    #[token("/=")]
    SlashEqual,

    #[token("%=")]
    ModuloEqual,

    #[token("+=")]
    PlusEqual,

    #[token("!=")]
    NotEqual,

    #[token("-=")]
    MinusEqual,

    #[token("<<=")]
    LeftShiftEqual,

    #[token(">>=")]
    RightShiftEqual,

    #[token("&=")]
    BitwiseAndEqual,

    #[token("|=")]
    BitwiseOrEqual,

    #[token("^=")]
    CaretEqual,

    #[token("^")]
    Caret,

    #[token("!")]
    Exclamation,

    #[token("~")]
    Tilde,

    #[token("<<")]
    LeftShift,

    #[token(">>")]
    RightShift,

    #[token("[")]
    LBrac,

    #[token("]")]
    RBrac,

    #[token("?")]
    Question,

    // Reserved words
    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("do")]
    Do,

    #[token("else")]
    Else,

    #[token("for")]
    For,

    #[token("if")]
    If,

    #[token("return")]
    Return,

    #[token("while")]
    While,

    #[token("int")]
    Int,

    #[token("char")]
    Char,

    #[token("long")]
    Long,

    #[token("short")]
    Short,

    #[token("void")]
    Void,

    #[token("double")]
    Double,

    #[token("struct")]
    Struct,

    #[token("typedef")]
    Typedef,

    #[token("const")]
    Const,

    #[token("case")]
    Case,

    #[token("default")]
    Default,

    #[token("switch")]
    Switch,

    #[token("goto")]
    Goto,

    // Regexes
    #[regex(r"[0-9]+", from_num)]
    Number(i64),

    #[regex("[_a-zA-Z][_a-zA-Z0-9]*")]
    Identifier(&'a str),

    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,

    // Comments
    #[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", logos::skip)]
    BlockComment,
    
    #[regex(r"//[^\n]*", logos::skip)]
    LineComment,

    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,

    #[error]
    Error
}

impl Display for Token<'static> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}