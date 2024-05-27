// AST Entry point
pub enum Program {
    Empty,
    GlobalUnit(Box<GlobalUnit>),
}

pub enum GlobalUnit {
    Function(Box<Function>),
    Struct(Box<Struct>),
    Typedef(Box<Typedef>),
}

pub enum Function {
    Declaration(Typ, Ident, Box<ParamList>),
    Definition(Typ, Ident, Box<ParamList>, Box<Statement>),
}

pub enum ParamList {
    Empty,
    List(Typ, Ident, Box<ParamList>),
}

pub enum Struct {
    Declaration(StructIdent),
    Definition(StructIdent, Box<StructMembers>)
}

pub enum StructMembers {
    List(Box<StructDeclaratorList>)
}

pub enum StructDeclaratorList {
    Empty,
    ListFollow(StructDeclarator, Box<StructDeclaratorList>)
}

pub enum StructDeclarator {
    Declaration(Typ, Ident)
}

pub enum Typedef {
    Declaration(Typ, Ident)
}

pub enum Statement {
    // Control
    IfElse(Box<Expression>, Box<Statement>, Option<Box<Statement>>),
    While(Box<Expression>, Box<Statement>),
    For(Box<Expression>, Box<Expression>, Box<Expression>, Box<Statement>),

    // Non-control
    Simple(Box<Statement>),
    Compound(Box<Statement>),
    Return(Box<Expression>),

    // Simple
    Assignment(Box<Expression>, AssignOp, Box<Expression>),
    Declaration(Typ, Ident),
    Definition(Typ, Ident, Box<Expression>),

    // Compound
    List(Box<StatementList>)
}

pub enum StatementList {
    Empty,
    ListFollow(Box<Statement>, Box<StatementList>)
}

pub enum Expression {
    FunctionCall(Ident, Box<ArgumentList>),
    Variable(Ident),
    Constant(i32),
    ArrayAccess(Box<Expression>, Box<Expression>),
    MemberAccess(Box<Expression>, Ident),
    Dereference(Box<Expression>),
    AddressOf(Box<Expression>),
    UnaryOp(UnaryOp, Box<Expression>),
    BinOp(Box<Expression>, BinaryOp, Box<Expression>),
}

pub enum ArgumentList {
    Empty,
    List(Box<Expression>, Box<ArgumentList>),
}

pub enum UnaryOp {
    Negate,
    Not,
    BitwiseNot
}

pub enum AssignOp {
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    ModuloEqual,
    LeftShiftEqual,
    RightShiftEqual,
    BitwiseAndEqual,
    BitwiseXorEqual,
    BitwiseOrEqual
}

pub enum BinaryOp {
    Multiply,
    Divide,
    Modulo,
    Plus,
    Minus,
    LeftShift,
    RightShift,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Equal,
    NotEqual,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LogicalAnd,
    LogicalOr
}

pub enum PostOp {
    PlusPlus,
    MinusMinus
}

pub enum Typ {
    Void,
    Int,
    Char,
    Long,
    Double,
    Bool,
    Signed,
    Unsigned,
    Struct(Ident),
    UserDefined(Ident)
}

pub type Ident = String;
pub type StructIdent = String;
pub type Number = i32;

