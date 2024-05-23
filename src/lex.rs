use logos::Logos;

pub enum Token {

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

    #[token("++")]
    PlusPlus,

    #[token("--")]
    MinusMinus,

    #[token("->")]
    Arrow,

    #[token("=>")]
    GreaterThanEqualTo,

    #[token(">")]
    GreaterThan,

    #[token("<")]
    LessThan,

    #[token("<=")]
    LessThanEqualTo,

    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token("&")]
    Ampersand,

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
    #[regex(r"[0-9]+")]
    Number,

    #[regex("[_a-zA-Z][_a-zA-Z0-9]*")]
    Identifier,

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