// AST Entry point
#[derive(Debug, Clone)]
pub enum Program {
    Empty,
    GlobalUnit(Box<GlobalUnit>, Box<Program>),
}

#[derive(Debug, Clone)]
pub enum GlobalUnit {
    Function(Box<Function>),
    Struct(Box<Struct>),
    Typedef(Box<Typedef>),
}

#[derive(Debug, Clone)]
pub enum Function {
    Declaration(Typ, Ident, Box<ParamList>),
    Definition(Typ, Ident, Box<ParamList>, Box<Statement>),
}

#[derive(Debug, Clone)]
pub enum ParamList {
    Empty,
    List(Typ, Ident, Box<ParamList>),
}

#[derive(Debug, Clone)]
pub enum Struct {
    Declaration(StructIdent),
    Definition(StructIdent, Box<StructMembers>)
}

#[derive(Debug, Clone)]
pub enum StructMembers {
    List(Box<StructDeclaratorList>)
}

#[derive(Debug, Clone)]
pub enum StructDeclaratorList {
    Empty,
    ListFollow(StructDeclarator, Box<StructDeclaratorList>)
}

#[derive(Debug, Clone)]
pub enum StructDeclarator {
    Declaration(Typ, Ident)
}

#[derive(Debug, Clone)]
pub enum Typedef {
    Declaration(Typ, Ident)
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum StatementList {
    Empty,
    ListFollow(Box<Statement>, Box<StatementList>)
}

#[derive(Debug, Clone)]
pub enum Expression {
    EmptyVar,
    FunctionCall(Ident, Box<ArgumentList>),
    Variable(Ident),
    Constant(i64),
    ArrayAccess(Box<Expression>, Box<Expression>),
    MemberAccess(Box<Expression>, Ident),
    Dereference(Box<Expression>),
    AddressOf(Box<Expression>),
    UnaryOp(UnaryOp, Box<Expression>),
    BinaryOp(Box<Expression>, BinaryOp, Box<Expression>),
}

#[derive(Debug, Clone)]
pub enum ArgumentList {
    Empty,
    List(Box<Expression>, Box<ArgumentList>),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Negate,
    Not,
    BitwiseNot
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

