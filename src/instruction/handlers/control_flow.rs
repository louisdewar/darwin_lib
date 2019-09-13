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

pub fn jmz(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &[Instruction],
) -> Option<usize> {
    use Modifier as m;

    // We completely ignore the modifier and the b mode
    let Instruction {
        a_reg,
        a_mode,
        b_reg,
        b_mode,
        modifier,
        ..
    } = instruction;

    // The index of the address that will be tested (b reg)
    let test_index = follow_address(b_reg, b_mode, cur_address, max, memory);

    // Match arm will return true if it should jump (false otherwise)
    if match modifier {
        m::A | m::BA => memory[test_index].a_reg == 0,
        m::B | m::AB => memory[test_index].b_reg == 0,
        m::F | m::X | m::I => memory[test_index].a_reg == 0 && memory[test_index].b_reg == 0,
        m::None => panic!("Invalid modifier `None` for JMZ"),
    } {
        Some(follow_address(a_reg, a_mode, cur_address, max, memory))
    } else {
        None
    }
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
