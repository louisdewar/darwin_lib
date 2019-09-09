use darwin_lib::{cmd, create_program, VirtualMachine};

#[test]
fn create_imp() {
    use darwin_lib::{AddressMode, Instruction, Modifier, OpCode};
    assert_eq!(
        create_program! { MOV(I, 0, Direct, 1, Direct) },
        vec!(Instruction::new(
            OpCode::MOV,
            Modifier::I,
            0,
            AddressMode::Direct,
            1,
            AddressMode::Direct
        ))
    )
}

#[test]
fn run_imp() {
    let program = create_program! { MOV(I, 0, Direct, 1, Direct) };
    let mut vm = VirtualMachine::new(20, program);

    let mov_instruction = cmd! { MOV(I, 0, Direct, 1, Direct) };

    let len = vm
        .get_memory()
        .iter()
        .filter(|instruction| **instruction == mov_instruction)
        .count();

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
            let len = vm
                .get_memory()
                .iter()
                .filter(|instruction| **instruction == mov_instruction)
                .count();

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
            .find(|instruction| **instruction != mov_instruction)
            == None,
        "The VM was not filled with MOV 0 1!"
    );

    println!("VM: {:?}", vm);
}
