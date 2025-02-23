// src/middle/irgen.rs

use crate::frontend::ast::{BinOp as ASTBinOp, Decl, Expr, Stmt};
use crate::middle::ir::{BinOp, Instruction};

pub struct IRGenerator {
    next_temp: usize,
}

impl IRGenerator {
    pub fn new() -> Self {
        IRGenerator { next_temp: 0 }
    }

    // Generate a unique temporary variable name.
    fn next_temp(&mut self) -> String {
        let temp = format!("t{}", self.next_temp);
        self.next_temp += 1;
        temp
    }

    /// Lower an expression and return a temporary variable name holding the result
    /// along with the list of generated IR instructions.
    pub fn lower_expr(&mut self, expr: &Expr) -> (String, Vec<Instruction>) {
        match expr {
            Expr::Literal(val) => {
                let temp = self.next_temp();
                let instr = Instruction::LoadConst {
                    dest: temp.clone(),
                    value: *val,
                };
                (temp, vec![instr])
            }
            Expr::Var(name) => {
                // In a more complete compiler, variable lookup would be done here.
                (name.clone(), vec![])
            }
            Expr::BinaryOp(left, op, right) => {
                let (left_temp, mut left_instrs) = self.lower_expr(left);
                let (right_temp, mut right_instrs) = self.lower_expr(right);
                let result_temp = self.next_temp();
                let ir_op = match op {
                    ASTBinOp::Add => BinOp::Add,
                    ASTBinOp::Sub => BinOp::Sub,
                    ASTBinOp::Mul => BinOp::Mul,
                    ASTBinOp::Div => BinOp::Div,
                };
                let bin_instr = Instruction::BinaryOp {
                    dest: result_temp.clone(),
                    op: ir_op,
                    left: left_temp,
                    right: right_temp,
                };
                let mut instructions = Vec::new();
                instructions.append(&mut left_instrs);
                instructions.append(&mut right_instrs);
                instructions.push(bin_instr);
                (result_temp, instructions)
            }
            Expr::Assign(var_expr, rhs) => {
                // We expect the left-hand side to be a variable.
                if let Expr::Var(var_name) = &**var_expr {
                    let (rhs_temp, mut rhs_instrs) = self.lower_expr(rhs);
                    let assign_instr = Instruction::Assign {
                        dest: var_name.clone(),
                        src: rhs_temp,
                    };
                    rhs_instrs.push(assign_instr);
                    (var_name.clone(), rhs_instrs)
                } else {
                    panic!("Left-hand side of assignment must be a variable");
                }
            }
        }
    }

    fn lower_if_else(&mut self, condition: &Expr, then_block: &[Stmt], else_block: &[Stmt]) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        // Lower the condition expression.
        let (cond_temp, mut cond_instrs) = self.lower_expr(condition);
        instructions.append(&mut cond_instrs);

        // Create unique labels.
        let then_label = self.next_label();
        let else_label = self.next_label();
        let end_label  = self.next_label();

        // Insert branch instruction.
        instructions.push(Instruction::Branch { 
            cond: cond_temp, 
            true_label: then_label.clone(), 
            false_label: else_label.clone() 
        });

        // Then block.
        instructions.push(Instruction::Label(then_label));
        for stmt in then_block {
            instructions.extend(self.lower_stmt(stmt));
        }
        instructions.push(Instruction::Jump(end_label.clone()));

        // Else block.
        instructions.push(Instruction::Label(else_label));
        for stmt in else_block {
            instructions.extend(self.lower_stmt(stmt));
        }
        instructions.push(Instruction::Jump(end_label.clone()));

        // End label.
        instructions.push(Instruction::Label(end_label));

        instructions
    }

    // A helper to generate unique labels.
    fn next_label(&mut self) -> String {
        let label = format!("L{}", self.next_temp);
        self.next_temp += 1;
        label
    }

    fn lower_while(&mut self, condition: &Expr, body: &[Stmt]) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        // Create unique labels.
        let cond_label = self.next_label();
        let body_label = self.next_label();
        let end_label  = self.next_label();

        // Insert branch instruction.
        instructions.push(Instruction::Jump(cond_label.clone()));
        instructions.push(Instruction::Label(cond_label.clone()));

        // Lower the condition expression.
        let (cond_temp, mut cond_instrs) = self.lower_expr(condition);
        instructions.append(&mut cond_instrs);

        // Insert branch instruction.
        instructions.push(Instruction::Branch { 
            cond: cond_temp, 
            true_label: body_label.clone(), 
            false_label: end_label.clone() 
        });

        // Body block.
        instructions.push(Instruction::Label(body_label));
        for stmt in body {
            instructions.extend(self.lower_stmt(stmt));
        }
        instructions.push(Instruction::Jump(cond_label.clone()));

        // End label.
        instructions.push(Instruction::Label(end_label));

        instructions
    }

    fn lower_for(&mut self, init: &Stmt, condition: &Expr, increment: &Expr, body: &[Stmt]) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        // Create unique labels.
        let cond_label = self.next_label();
        let body_label = self.next_label();
        let end_label  = self.next_label();

        // Lower the init statement.
        instructions.extend(self.lower_stmt(init));

        // Insert branch instruction.
        instructions.push(Instruction::Jump(cond_label.clone()));
        instructions.push(Instruction::Label(cond_label.clone()));

        // Lower the condition expression.
        let (cond_temp, mut cond_instrs) = self.lower_expr(condition);
        instructions.append(&mut cond_instrs);

        // Insert branch instruction.
        instructions.push(Instruction::Branch { 
            cond: cond_temp, 
            true_label: body_label.clone(), 
            false_label: end_label.clone() 
        });

        // Body block.
        instructions.push(Instruction::Label(body_label));
        for stmt in body {
            instructions.extend(self.lower_stmt(stmt));
        }

        // Lower the increment expression.
        let (inc_temp, mut inc_instrs) = self.lower_expr(increment);
        instructions.append(&mut inc_instrs);

        instructions.push(Instruction::Jump(cond_label.clone()));

        // End label.
        instructions.push(Instruction::Label(end_label));

        instructions
    }

    /// Lower a statement into IR instructions.
    pub fn lower_stmt(&mut self, stmt: &Stmt) -> Vec<Instruction> {
        match stmt {
            Stmt::Expr(expr) => {
                let (_temp, instrs) = self.lower_expr(expr);
                instrs
            }
            Stmt::Decl(decl) => self.lower_decl(decl),

            Stmt::If { condition, then_block } => {
                self.lower_if_else(condition, then_block, &[])
            }
            Stmt::IfElse { condition, then_block, else_block } => {
                self.lower_if_else(condition, then_block, else_block)
            }

            Stmt::While { condition, body } => {
                self.lower_while(condition, body)
            }

            Stmt::For { init, condition, increment, body } => {
                self.lower_for(init, condition, increment, body)
            }

        }
    }

    /// Lower a declaration statement.
    pub fn lower_decl(&mut self, decl: &Decl) -> Vec<Instruction> {
        match decl {
            Decl::VarDecl { name, init, .. } => {
                if let Some(expr) = init {
                    let (temp, mut instrs) = self.lower_expr(expr);
                    // Declare the variable and then assign the computed value.
                    instrs.push(Instruction::VarDecl {
                        name: name.clone(),
                        src: Some(temp),
                    });
                    instrs
                } else {
                    vec![Instruction::VarDecl {
                        name: name.clone(),
                        src: None,
                    }]
                }
            }
        }
    }

    /// Lower an entire program (list of statements) into IR.
    pub fn lower_program(&mut self, stmts: &[Stmt]) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        for stmt in stmts {
            instructions.extend(self.lower_stmt(stmt));
        }
        instructions
    }
}
