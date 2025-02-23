// src/frontend/ast.rs

#[derive(Debug)]
pub enum Program {
    Stmts(Vec<Stmt>),
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Decl(Decl),
    If { 
        condition: Expr, 
        then_block: Vec<Stmt>,
    },
    IfElse { 
        condition: Expr, 
        then_block: Vec<Stmt>,
        else_block: Vec<Stmt>,
    },
    While { 
        condition: Expr, 
        body: Vec<Stmt>,
    },
    For { 
        init: Box<Stmt>, 
        condition: Expr, 
        increment: Expr, 
        body: Vec<Stmt>,
    },
    // Future: additional control flow or statements.
}


#[derive(Debug)]
pub enum Decl {
    VarDecl {
        typ: Type,
        name: String,
        init: Option<Expr>,
    },
}

#[derive(Debug)]
pub enum Expr {
    Literal(i32),
    Var(String),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    Assign(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Type {
    Int,
    // Future extensions: bool, string, pointers, etc.
}
