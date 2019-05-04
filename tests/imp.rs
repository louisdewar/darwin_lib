use darwin_lib::{Instruction, InstructionType::*, VirtualMachine};

#[test]
fn run_imp() {
    let mov_instruction = Instruction::new(MOV, 0, 1);
    let program = vec![mov_instruction];
    let mut vm = VirtualMachine::new(20, program);

    let len = (vm
        .get_memory()
        .iter()
        .filter(|instruction| **instruction == mov_instruction)
        .collect::<Vec<_>>())
    .len();
    assert!(
        len == 1,
        "The VM was not initialised with exactly 1 MOV 0 1! Got {} instead",
        len
    );

    // Cycle 19 times (should fill up vm with MOV 0 1)
    for i in 0..19 {
        vm.cycle();

        // At this point there should be 6 MOV 0 1 (1st one + 5 copies)
        if i == 4 {
            let len = (vm
                .get_memory()
                .iter()
                .filter(|instruction| **instruction == mov_instruction)
                .collect::<Vec<_>>())
            .len();
            assert!(
                len == 6,
                "The VM was not filled with exactly 6 MOV 0 1! Got {} instead",
                len
            );
        }
    }

    assert!(
        vm.get_memory()
            .iter()
            .filter(|instruction| **instruction != mov_instruction)
            .next()
            == None,
        "The VM was not filled with MOV 0 1!"
    );

    println!("VM: {:?}", vm);
}
