#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstructionType {
    /// Copy instruction from source (a) to address (b)
    MOV,
    ADD,
    DAT,
    JMP
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub instruction_type: InstructionType,
    pub reg_a: usize,
    pub reg_b: usize,
}

impl Instruction {
    pub fn new(instruction_type: InstructionType, reg_a: usize, reg_b: usize) -> Instruction {
        Instruction {
            instruction_type,
            reg_a,
            reg_b,
        }
    }
}
