use crate::{Instruction, Modifier};

use super::follow_address;

/// Helper function that returns the source and destination addresses as a tuple
fn get_source_destination(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) -> (usize, usize) {
    let Instruction {
        a_reg,
        a_mode,
        b_reg,
        b_mode,
        ..
    } = instruction;

    let source = follow_address(a_reg, a_mode, cur_address, max, memory);
    let destination = follow_address(b_reg, b_mode, cur_address, max, memory);
    (source, destination)
}

fn perform_operation(
    modifier: Modifier,
    mem_source: Instruction,
    mem_destination: &mut Instruction,
    max: isize,
    operation: Box<dyn Fn(isize, isize) -> isize>,
) {
    use Modifier as m;

    match modifier {
        m::A => {
            mem_destination.a_reg = (operation)(mem_source.a_reg, mem_destination.a_reg);
            mem_destination.a_reg = (mem_destination.a_reg + max) % max;
        }
        m::B => {
            mem_destination.b_reg = (operation)(mem_source.b_reg, mem_destination.b_reg);
            mem_destination.b_reg = (mem_destination.b_reg + max) % max;
        }
        // This is the default implementation
        m::AB | m::None => {
            mem_destination.b_reg = (operation)(mem_source.a_reg, mem_destination.b_reg);
            mem_destination.b_reg = (mem_destination.b_reg + max) % max;
        }
        m::BA => {
            mem_destination.a_reg = (operation)(mem_source.b_reg, mem_destination.a_reg);
            mem_destination.a_reg = (mem_destination.a_reg + max) % max;
        }
        m::X => {
            mem_destination.a_reg = (operation)(mem_source.b_reg, mem_destination.a_reg);
            mem_destination.b_reg = (operation)(mem_source.a_reg, mem_destination.b_reg);
            mem_destination.a_reg = (mem_destination.a_reg + max) % max;
            mem_destination.b_reg = (mem_destination.b_reg + max) % max;
        }
        m::F | m::I => {
            mem_destination.a_reg = (operation)(mem_source.a_reg, mem_destination.a_reg);
            mem_destination.b_reg = (operation)(mem_source.b_reg, mem_destination.b_reg);
            mem_destination.a_reg = (mem_destination.a_reg + max) % max;
            mem_destination.b_reg = (mem_destination.b_reg + max) % max;
        }
    }
}

pub fn add(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) {
    let (source, destination) = get_source_destination(instruction, cur_address, max, memory);
    perform_operation(
        instruction.modifier,
        memory[source],
        &mut memory[destination],
        max as isize,
        Box::new(|s, d| d + s),
    );
}

pub fn sub(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) {
    let (source, destination) = get_source_destination(instruction, cur_address, max, memory);
    perform_operation(
        instruction.modifier,
        memory[source],
        &mut memory[destination],
        max as isize,
        Box::new(|s, d| d - s),
    );
}

pub fn mul(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) {
    let (source, destination) = get_source_destination(instruction, cur_address, max, memory);

    perform_operation(
        instruction.modifier,
        memory[source],
        &mut memory[destination],
        max as isize,
        Box::new(|s, d| d * s),
    );
}

pub fn div(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) {
    let (source, destination) = get_source_destination(instruction, cur_address, max, memory);

    perform_operation(
        instruction.modifier,
        memory[source],
        &mut memory[destination],
        max as isize,
        Box::new(|s, d| d / s),
    );
}

pub fn modulo(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) {
    let (source, destination) = get_source_destination(instruction, cur_address, max, memory);

    perform_operation(
        instruction.modifier,
        memory[source],
        &mut memory[destination],
        max as isize,
        Box::new(|s, d| d % s),
    );
}
