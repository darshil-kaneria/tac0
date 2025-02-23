use crate::frontend::ast::{BinOp as ASTBinOp, Decl, Expr, Program, Stmt};
use crate::middle::ir::{BinOp, Instruction};

pub struct IRGenerator {
    next_temp: usize,
}

impl IRGenerator {
    pub fn new() -> Self {
        IRGenerator { next_temp: 0 }
    }

    fn next_temp(&mut self) -> String {
        let temp = format!("t{}", self.next_temp);
        self.next_temp += 1;
        temp
    }

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
                    ASTBinOp::Lt => BinOp::Lt,
                    ASTBinOp::Le => BinOp::Le,
                    ASTBinOp::Gt => BinOp::Gt,
                    ASTBinOp::Ge => BinOp::Ge,
                    ASTBinOp::Eq => BinOp::Eq,
                    ASTBinOp::Neq => BinOp::Neq,
                    ASTBinOp::And => BinOp::And,
                    ASTBinOp::Or => BinOp::Or,
                    ASTBinOp::Not => BinOp::Not,
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
            Expr::UnaryOp(op, expr) => {
                let (expr_temp, mut expr_instrs) = self.lower_expr(expr);
                let result_temp = self.next_temp();
                let ir_op = match op {
                    crate::frontend::ast::UnaryOp::Neg => BinOp::Sub,
                    crate::frontend::ast::UnaryOp::Not => BinOp::Not,
                };
                let unary_instr = Instruction::BinaryOp {
                    dest: result_temp.clone(),
                    op: ir_op,
                    left: "0".to_string(),
                    right: expr_temp,
                };
                expr_instrs.push(unary_instr);
                (result_temp, expr_instrs)
            }
            Expr::Assign(var_expr, rhs) => {
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

        let (cond_temp, mut cond_instrs) = self.lower_expr(condition);
        instructions.append(&mut cond_instrs);

        let then_label = self.next_label();
        let else_label = self.next_label();
        let end_label  = self.next_label();

        instructions.push(Instruction::Branch { 
            cond: cond_temp, 
            true_label: then_label.clone(), 
            false_label: else_label.clone() 
        });

        instructions.push(Instruction::Label(then_label));
        for stmt in then_block {
            instructions.extend(self.lower_stmt(stmt));
        }
        instructions.push(Instruction::Jump(end_label.clone()));

        instructions.push(Instruction::Label(else_label));
        for stmt in else_block {
            instructions.extend(self.lower_stmt(stmt));
        }
        instructions.push(Instruction::Jump(end_label.clone()));

        instructions.push(Instruction::Label(end_label));

        instructions
    }

    fn next_label(&mut self) -> String {
        let label = format!("L{}", self.next_temp);
        self.next_temp += 1;
        label
    }

    fn lower_while(&mut self, condition: &Expr, body: &[Stmt]) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let cond_label = self.next_label();
        let body_label = self.next_label();
        let end_label  = self.next_label();

        instructions.push(Instruction::Jump(cond_label.clone()));
        instructions.push(Instruction::Label(cond_label.clone()));

        let (cond_temp, mut cond_instrs) = self.lower_expr(condition);
        instructions.append(&mut cond_instrs);

        instructions.push(Instruction::Branch { 
            cond: cond_temp, 
            true_label: body_label.clone(), 
            false_label: end_label.clone() 
        });

        instructions.push(Instruction::Label(body_label));
        for stmt in body {
            instructions.extend(self.lower_stmt(stmt));
        }
        instructions.push(Instruction::Jump(cond_label.clone()));

        instructions.push(Instruction::Label(end_label));

        instructions
    }

    fn lower_for(&mut self, init: &Stmt, condition: &Expr, increment: &Expr, body: &[Stmt]) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let cond_label = self.next_label();
        let body_label = self.next_label();
        let end_label  = self.next_label();

        instructions.extend(self.lower_stmt(init));

        instructions.push(Instruction::Jump(cond_label.clone()));
        instructions.push(Instruction::Label(cond_label.clone()));

        let (cond_temp, mut cond_instrs) = self.lower_expr(condition);
        instructions.append(&mut cond_instrs);

        instructions.push(Instruction::Branch { 
            cond: cond_temp, 
            true_label: body_label.clone(), 
            false_label: end_label.clone() 
        });

        instructions.push(Instruction::Label(body_label));
        for stmt in body {
            instructions.extend(self.lower_stmt(stmt));
        }

        let (inc_temp, mut inc_instrs) = self.lower_expr(increment);
        instructions.append(&mut inc_instrs);

        instructions.push(Instruction::Jump(cond_label.clone()));

        instructions.push(Instruction::Label(end_label));

        instructions
    }

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

    pub fn lower_decl(&mut self, decl: &Decl) -> Vec<Instruction> {
        match decl {
            Decl::VarDecl { name, init, .. } => {
                if let Some(expr) = init {
                    let (temp, mut instrs) = self.lower_expr(expr);
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

    pub fn lower_program(&mut self, prog: &Program) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        match prog {
            Program::Stmts(stmts) => {
                for stmt in stmts {
                    instructions.extend(self.lower_stmt(stmt));
                }
            }
        }
        instructions
    }
}
