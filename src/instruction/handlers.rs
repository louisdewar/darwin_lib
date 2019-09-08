use crate::{AddressMode, Instruction, Modifier};

#[inline]
pub fn relative_address(max: usize, a: usize, b: isize) -> usize {
    let max = max as isize;
    let a = a as isize;
    // If a + b is negative then the result of ((a + b) % max) will be negative so we must add max
    // If ((a + b) % max) is 0, then + max, will result in max, therefore we need to % max again to convert back to 0
    ((((a + b) % max) + max) % max) as usize
}

#[inline]
pub fn follow_address(
    reg: isize,
    mode: AddressMode,
    cur_address: usize,
    max: usize,
    memory: &[Instruction],
) -> usize {
    use AddressMode::*;

    match mode {
        // Direct will always return an insutrction
        Direct => relative_address(max, cur_address, reg),
        // Immediate will always return a value
        Immediate => cur_address,
        Indirect => {
            let index = relative_address(max, cur_address, reg);
            let instruction = memory[index];
            relative_address(max, index, instruction.b_reg)
        }
    }
}

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
