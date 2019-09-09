use crate::{Instruction, Modifier};

use super::follow_address;

pub fn mov(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) {
    use Modifier as m;

    let Instruction {
        a_reg,
        a_mode,
        b_reg,
        b_mode,
        modifier,
        ..
    } = instruction;

    match modifier {
        m::I => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination] = memory[source];
        }
        m::A => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].a_reg = memory[source].a_reg;
        }
        m::B => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].b_reg = memory[source].b_reg;
        }
        m::AB => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].b_reg = memory[source].a_reg;
        }
        m::BA => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].a_reg = memory[source].b_reg;
        }
        m::F => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].a_reg = memory[source].a_reg;
            memory[destination].b_reg = memory[source].b_reg;
        }
        m::X => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].a_reg = memory[source].b_reg;
            memory[destination].b_reg = memory[source].a_reg;
        }
        m::None => panic!("Invalid modifier None for mov"),
    }
}

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
