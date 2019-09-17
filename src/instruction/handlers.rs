use crate::{AddressMode, Instruction};

mod control_flow;
pub use control_flow::*;

mod general;
pub use general::*;

mod operators;
pub use operators::*;

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
        IndirectA | PreDecrementIndirectA | PostDecrementIndirectA => {
            let index = relative_address(max, cur_address, reg);
            let instruction = memory[index];
            relative_address(max, index, instruction.a_reg)
        }
        IndirectB | PreDecrementIndirectB | PostDecrementIndirectB => {
            let index = relative_address(max, cur_address, reg);
            let instruction = memory[index];
            relative_address(max, index, instruction.b_reg)
        }
    }
}
