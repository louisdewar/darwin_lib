pub mod handlers;

mod util;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressMode {
    /// The number following this operand points (relatively) to the address of the value
    Direct,
    /// The number following this operand is the value
    Immediate,
    /// The number following this operand points to the location where a relative pointer to the value can be found
    Indirect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// The type of instruction
pub enum OpCode {
    MOV,
    DAT,
    JMP,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// The modifier of an instruction
pub enum Modifier {
    None,
    A,
    B,
    AB,
    BA,
    F,
    X,
    I,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// The structure representing a sinlge instruction at a point in memory
pub struct Instruction {
    /// The op code (the type of instruction)
    pub op_code: OpCode,
    /// The modifier of the instruction
    pub modifier: Modifier,

    /// The value of the a register
    pub a_reg: isize,
    /// The address mode of the a register
    pub a_mode: AddressMode,

    /// The value of the b register
    pub b_reg: isize,
    /// The address mode of the b register
    pub b_mode: AddressMode,
}

impl Instruction {
    pub fn new(
        op_code: OpCode,
        modifier: Modifier,
        a_reg: isize,
        a_mode: AddressMode,
        b_reg: isize,
        b_mode: AddressMode,
    ) -> Instruction {
        Instruction {
            op_code,
            modifier,
            a_reg,
            a_mode,
            b_reg,
            b_mode,
        }
    }
}
