use crate::middle::ir::*;
pub fn print_ir(ir: Vec<Instruction>) -> Vec<Instruction> {
    for elem in ir.iter() {
        println!("{:?}", elem);
    }

    ir
}