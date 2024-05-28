use logos::Logos;
use std::fmt;

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
    BitwiseXorEqual,

    #[token("^")]
    BitwiseXor,

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

    #[token("unsigned")]
    Unsigned,

    #[token("bool")]
    Bool,

    #[token("signed")]
    Signed,

    #[token("void")]
    Void,

    #[token("true")]
    True,

    #[token("false")]
    False,

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
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
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
    Error,
}

impl<'a> fmt::Display for Token<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:#?}", self)
  }
}