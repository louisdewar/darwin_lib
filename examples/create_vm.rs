use darwin_lib::{Instruction, InstructionType::*, VirtualMachine};

fn main() {
    let program = vec![Instruction::new(MOV, 0, 1)];

    let mut vm = VirtualMachine::new(20, program);

    vm.cycle();
    vm.cycle();

    println!("VM: {:?}", vm);
}
