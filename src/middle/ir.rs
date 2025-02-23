// src/middle/ir.rs

#[derive(Debug)]
pub enum Instruction {
    // Load a constant integer into a temporary variable.
    LoadConst { dest: String, value: i32 },
    // Perform a binary operation between two operands.
    BinaryOp { dest: String, op: BinOp, left: String, right: String },
    // Assign the value of one temporary to another variable.
    Assign { dest: String, src: String },
    // Store a variable’s initial value (for declarations with initialization).
    VarDecl { name: String, src: Option<String> },

    Label(String),
    Jump(String),
    Branch { cond: String, true_label: String, false_label: String },
}

// Reuse our binary operators in the IR.
#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
