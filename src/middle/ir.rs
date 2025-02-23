#[derive(Debug)]
pub enum Instruction {
    LoadConst { dest: String, value: i32 },
    BinaryOp { dest: String, op: BinOp, left: String, right: String },
    Assign { dest: String, src: String },
    VarDecl { name: String, src: Option<String> },

    Label(String),
    Jump(String),
    Branch { cond: String, true_label: String, false_label: String },
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Neq,
    And,
    Or,
    Not,
}
