

// The AST "entry point"
pub enum Program {
    ExternalDeclaration(Box<ExternalDeclaration>),
}

pub enum ExternalDeclaration {
    FunctionDefinition(Box<FunctionDefinition>),
    Declaration(Box<Declaration>),
}

pub struct FunctionDefinition {
    pub declaration_specifiers: Vec<DeclarationSpecifier>,
    pub declarator: Declarator,
    pub declaration: Vec<Declaration>,
    pub compound_statement: CompoundStatement
}

pub enum DeclarationSpecifier {
    StorageClassSpecifier(Box<StorageClassSpecifier>),
    TypeSpecifier(Box<Typ>),
    TypeQualifer(Box<TypQualifier>)
}

// Because Type isn't available
pub enum Typ {
    Void,
    Char,
    Short,
    Int,
    Long,
    Float,
    Double,
    Signed,
    Unsigned,
    Typedef(String),
    StructOrUnion(Box<StructOrUnion>),
    Enum(Box<Enum>),
}

pub enum TypQualifier {
    Const,
    Volatile,
}

pub struct StructOrUnionSpecifier {
    pub struct_or_union: StructOrUnion,
    pub identifier: Option<String>,
    pub struct_declaration: Option<Vec<StructDeclaration>>,
}

pub enum StructOrUnion {
    Struct,
    Union,
}

pub struct StructDeclaration {
    pub specifier_qualifiers: Vec<SpecifierQualifier>,
    pub struct_declarator_list: StructDeclaratorList
}

pub enum StorageClassSpecifier {
    Typedef,
    Extern,
    Static,
    Auto,
    Register,
}

pub enum Boolean {
    True,
    False,
}
// Either specify a type or a qualifier
pub enum SpecifierQualifier {
    TypeQualifer(Box<TypQualifier>),
    TypeSpecifier(Box<Typ>),
}

pub enum StructDeclaratorListFollow {
    Empty,
    List(Box<StructDeclarator>, Box<StructDeclaratorListFollow>),
}

pub enum StructDeclaratorList {
    Empty,
    List(Box<StructDeclarator>, Box<StructDeclaratorListFollow>),
}

pub enum StructDeclarator {

}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // Logical Operators
    LogicalOr,
    LogicalAnd,
    
    // Bitwise Operators
    BitwiseOr,
    BitwiseAnd,

    Charet,

    // Conditional Operators
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,

    // Shift Operators
    LeftShift,
    RightShift,
}

pub enum UnaryOp {
    
}

pub enum PostOp {
    PlusPlus,
    MinusMinus,
}


pub enum Expr {

}

pub enum Stmt {

}

