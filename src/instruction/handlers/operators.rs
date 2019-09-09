use crate::{Instruction, Modifier};

use super::follow_address;

pub fn add(
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
        m::A => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].a_reg += memory[source].a_reg;
        }
        m::B => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].b_reg += memory[source].b_reg;
        }
        // This is the default implementation
        m::AB | m::None => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].b_reg += memory[source].a_reg;
        }
        m::BA => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].a_reg += memory[source].b_reg;
        }
        m::X => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].a_reg += memory[source].b_reg;
            memory[destination].b_reg += memory[source].a_reg;
        }
        m::F | m::I => {
            let source = follow_address(a_reg, a_mode, cur_address, max, memory);
            let destination = follow_address(b_reg, b_mode, cur_address, max, memory);

            memory[destination].a_reg += memory[source].a_reg;
            memory[destination].b_reg += memory[source].b_reg;
        }
    }
}
