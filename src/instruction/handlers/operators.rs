use crate::{Instruction, Modifier};

use super::follow_address;

/// Performs an arithmetic operation between two memory locations. Should not be used if operation
/// can result in division by zero.
macro_rules! perform_operation {
    ($modifier:expr, $mem_source:expr, $mem_destination:expr, $max:expr, $op:tt) => {
        match $modifier {
            Modifier::A => {
                $mem_destination.a_reg $op $mem_source.a_reg;
                $mem_destination.a_reg = ($mem_destination.a_reg + $max) % $max;
            },
            Modifier::B => {
                $mem_destination.b_reg $op $mem_source.b_reg;
                $mem_destination.b_reg = ($mem_destination.b_reg + $max) % $max;
            },
            // This is the default implementation
            Modifier::AB | Modifier::None => {
                $mem_destination.b_reg $op $mem_source.a_reg;
                $mem_destination.b_reg = ($mem_destination.b_reg + $max) % $max;
            },
            Modifier::BA => {
                $mem_destination.a_reg $op $mem_source.b_reg;
                $mem_destination.a_reg = ($mem_destination.a_reg + $max) % $max;
            },
            Modifier::X => {
                $mem_destination.a_reg $op $mem_source.b_reg;
                $mem_destination.b_reg $op $mem_source.a_reg;
                $mem_destination.a_reg = ($mem_destination.a_reg + $max) % $max;
                $mem_destination.b_reg = ($mem_destination.b_reg + $max) % $max;
            },
            Modifier::F | Modifier::I => {
                $mem_destination.a_reg $op $mem_source.a_reg;
                $mem_destination.b_reg $op $mem_source.b_reg;
                $mem_destination.a_reg = ($mem_destination.a_reg + $max) % $max;
                $mem_destination.b_reg = ($mem_destination.b_reg + $max) % $max;
            }
        }
    }
}

macro_rules! try_op {
    ($expr: expr) => {
        match $expr {
            Some(x) => x,
            None => return false,
        }
    };
}

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

pub fn add(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) {
    let (source, destination) = get_source_destination(instruction, cur_address, max, memory);
    perform_operation!(
        instruction.modifier,
        memory[source],
        &mut memory[destination],
        max as isize,
        +=
    );
}

pub fn sub(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) {
    let (source, destination) = get_source_destination(instruction, cur_address, max, memory);
    perform_operation!(
        instruction.modifier,
        memory[source],
        &mut memory[destination],
        max as isize,
        -=
    );
}

pub fn mul(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) {
    let (source, destination) = get_source_destination(instruction, cur_address, max, memory);

    perform_operation!(
        instruction.modifier,
        memory[source],
        &mut memory[destination],
        max as isize,
        *=
    );
}

pub fn div(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) -> bool {
    let (source, destination) = get_source_destination(instruction, cur_address, max, memory);
    use Modifier as m;
    match instruction.modifier {
        m::A => {
            memory[destination].a_reg =
                try_op!(memory[destination].a_reg.checked_div(memory[source].a_reg));
        }
        m::B => {
            memory[destination].b_reg =
                try_op!(memory[destination].b_reg.checked_div(memory[source].b_reg));
        }
        m::AB | m::None => {
            memory[destination].b_reg =
                try_op!(memory[destination].b_reg.checked_div(memory[source].a_reg));
        }
        m::BA => {
            memory[destination].a_reg =
                try_op!(memory[destination].a_reg.checked_div(memory[source].b_reg));
        }
        m::X => {
            // If one of the divisors is 0, the other will still be done as normal.
            let result1 = memory[destination].b_reg.checked_div(memory[source].a_reg);
            let result2 = memory[destination].a_reg.checked_div(memory[source].b_reg);
            if let Some(t) = result1 {
                memory[destination].b_reg = t;
            }
            if let Some(t) = result2 {
                memory[destination].a_reg = t;
            }
            if result1 == None || result2 == None {
                return false;
            }
        }
        m::F | m::I => {
            // If one of the divisors is 0, the other will still be done as normal.
            let result1 = memory[destination].a_reg.checked_div(memory[source].a_reg);
            let result2 = memory[destination].b_reg.checked_div(memory[source].b_reg);
            if let Some(t) = result1 {
                memory[destination].a_reg = t;
            }
            if let Some(t) = result2 {
                memory[destination].b_reg = t;
            }
            if result1 == None || result2 == None {
                return false;
            }
        }
    }
    true
}

pub fn modulo(
    instruction: Instruction,
    cur_address: usize,
    max: usize,
    memory: &mut Vec<Instruction>,
) -> bool {
    let (source, destination) = get_source_destination(instruction, cur_address, max, memory);
    use Modifier as m;
    match instruction.modifier {
        m::A => {
            memory[destination].a_reg =
                try_op!(memory[destination].a_reg.checked_rem(memory[source].a_reg));
        }
        m::B => {
            memory[destination].b_reg =
                try_op!(memory[destination].b_reg.checked_rem(memory[source].b_reg));
        }
        m::AB | m::None => {
            memory[destination].b_reg =
                try_op!(memory[destination].b_reg.checked_rem(memory[source].a_reg));
        }
        m::BA => {
            memory[destination].a_reg =
                try_op!(memory[destination].a_reg.checked_rem(memory[source].b_reg));
        }
        m::X => {
            // If one of the divisors is 0, the other will still be done as normal.
            let result1 = memory[destination].b_reg.checked_rem(memory[source].a_reg);
            let result2 = memory[destination].a_reg.checked_rem(memory[source].b_reg);
            if let Some(t) = result1 {
                memory[destination].b_reg = t;
            }
            if let Some(t) = result2 {
                memory[destination].a_reg = t;
            }
            if result1 == None || result2 == None {
                return false;
            }
        }
        m::F | m::I => {
            // If one of the divisors is 0, the other will still be done as normal.
            let result1 = memory[destination].a_reg.checked_rem(memory[source].a_reg);
            let result2 = memory[destination].b_reg.checked_rem(memory[source].b_reg);
            if let Some(t) = result1 {
                memory[destination].a_reg = t;
            }
            if let Some(t) = result2 {
                memory[destination].b_reg = t;
            }
            if result1 == None || result2 == None {
                return false;
            }
        }
    }
    true
}
