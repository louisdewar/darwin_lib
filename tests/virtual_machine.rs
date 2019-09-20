use darwin_lib::{cmd, create_program, MatchSettings, VirtualMachine};

#[test]
fn random_insert() {
    let imp = create_program! {
        MOV(I, 0, Direct, 1, Direct)
    };

    let mut vm = VirtualMachine::new_battle(
        &[imp.clone(), imp.clone()],
        &MatchSettings {
            min_separation: 10,
            core_size: 22,
            ..Default::default()
        },
    );

    vm.cycle();

    let memory = vm.get_memory();

    // Due to the constraints of min_separation and core_size, even though new_battle randomly inserts
    // there is only 1 valid solution for this so each insertion point is known
    // Also since the VM is cycled once the program first inserted at index 10, should have created a copy at 11
    for i in &[10, 11, 21] {
        assert_eq!(
            memory[*i],
            cmd! { MOV(I, 0, Direct, 1, Direct) },
            "After one cycle memory was incorrect"
        )
    }

    let empty_count = memory
        .iter()
        .filter(|x| **x == cmd! { DAT(None, 0, Immediate, 0, Immediate) })
        .count();
    assert_eq!(
        empty_count,
        22 - 3,
        "The memory wasn't initialised correctly with DAT"
    )
}

#[should_panic]
#[test]
fn random_insert_not_enough_room() {
    let imp = create_program! {
        MOV(I, 0, Direct, 1, Direct)
    };

    // Core size should be greater than 21 otherwise not enough room
    VirtualMachine::new_battle(
        &[imp.clone(), imp.clone()],
        &MatchSettings {
            min_separation: 10,
            core_size: 21,
            ..Default::default()
        },
    );
}
