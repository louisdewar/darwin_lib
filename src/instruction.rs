pub mod handlers;
pub use handlers::relative_address;

mod util;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressMode {
    /// The number following this operand points (relatively) to the address of the value
    Direct,
    /// The number following this operand is the value
    Immediate,
    /// The number following this operand points to the location where a relative pointer to the
    /// value can be found in the A-register
    IndirectA,
    /// The number following this operand points to the location where a relative pointer to the
    /// value can be found in the B-register
    IndirectB,
    // Behaves like IndirectA, but it decrements the number it points to by 1 before running the instruction
    PreDecrementIndirectA,
    // Behaves like IndirectB, but it decrements the number it points to by 1 before running the instruction
    PreDecrementIndirectB,
}

impl std::fmt::Display for AddressMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use AddressMode::*;
        match self {
            Direct => write!(f, "$"),
            Immediate => write!(f, "#"),
            IndirectA => write!(f, "*"),
            IndirectB => write!(f, "@"),
            PreDecrementIndirectA => write!(f, "{{"),
            PreDecrementIndirectB => write!(f, "<"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// The type of instruction
pub enum OpCode {
    MOV,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    DAT,
    JMP,
    SPL,
    JMZ,
    JMN,
    NOP,
    DJN,
    SEQ,
    SNE,
    SLT,
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

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.modifier == Modifier::None {
            write!(
                f,
                "{:?}    {}{} {}{}",
                self.op_code, self.a_mode, self.a_reg, self.b_mode, self.b_reg
            )
        } else {
            let modifier = format!("{:?}", self.modifier);
            write!(
                f,
                "{:?}.{}{} {}{} {}{}",
                self.op_code,
                modifier,
                if modifier.len() == 1 { " " } else { "" },
                self.a_mode,
                self.a_reg,
                self.b_mode,
                self.b_reg
            )
        }
    }
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
