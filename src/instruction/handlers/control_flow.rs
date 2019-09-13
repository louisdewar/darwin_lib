use crate::{Instruction, Modifier};

use super::follow_address;

pub fn jmp(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &[Instruction],
) -> usize {
    // We completely ignore the modifier and the b mode
    let Instruction { a_reg, a_mode, .. } = instruction;

    follow_address(a_reg, a_mode, cur_address, max, memory)
}

pub fn spl(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &[Instruction],
) -> usize {
    // We completely ignore the modifier and the b mode
    let Instruction { a_reg, a_mode, .. } = instruction;

    follow_address(a_reg, a_mode, cur_address, max, memory)
}
