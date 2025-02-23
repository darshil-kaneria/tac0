/**
 * A very simple pass manager to apply passes to the IR.
 */
use crate::middle::ir::*;

pub type IRPass = fn(Vec<Instruction>) -> Vec<Instruction>;

pub struct PassManager {
    passes: Vec<IRPass>,
}

impl PassManager {
    pub fn new() -> Self {
        PassManager { passes: Vec::new() }
    }

    pub fn add_pass(&mut self, pass: IRPass) {
        self.passes.push(pass);
    }

    pub fn run(&self, ir: Vec<Instruction>) -> Vec<Instruction> {
        let mut current_ir = ir;
        for pass in &self.passes {
            current_ir = pass(current_ir);
        }
        current_ir
    }
}
