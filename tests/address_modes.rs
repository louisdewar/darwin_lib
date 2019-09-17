use darwin_lib::{
    cmd, create_program, handlers::follow_address, AddressMode::*, Instruction, VirtualMachine,
};

fn generate_empty_memory(size: usize) -> Vec<Instruction> {
    (0..size)
        .map(|_| cmd!(DAT(None, 0, Immediate, 0, Immediate)))
        .collect()
}

#[test]
fn immediate() {
    let memory = generate_empty_memory(10);

    assert_eq!(
        follow_address(4, Immediate, 1, memory.len(), &memory),
        1,
        "Immediate address mode should always point to the current instruction"
    );
    assert_eq!(
        follow_address(4, Immediate, 3, memory.len(), &memory),
        3,
        "Immediate address mode should always point to the current instruction"
    );
}

#[test]
fn direct() {
    let memory = generate_empty_memory(10);

    assert_eq!(follow_address(2, Direct, 1, memory.len(), &memory), 2 + 1);
    assert_eq!(follow_address(4, Direct, 3, memory.len(), &memory), 4 + 3);
}

#[test]
fn indirect_a() {
    let mut memory = generate_empty_memory(10);

    memory[0] = cmd! { DAT(None, 2, Immediate, 4, Immediate) };
    memory[1] = cmd! { DAT(None, 3, Immediate, 5, Immediate) };

    // Since memory is cyclic this should point to index 0 which in turn points to 2
    assert_eq!(follow_address(1, IndirectA, 9, memory.len(), &memory), 2);
    assert_eq!(
        follow_address(1, PreDecrementIndirectA, 9, memory.len(), &memory),
        2
    );
    assert_eq!(
        follow_address(1, PostDecrementIndirectA, 9, memory.len(), &memory),
        2
    );

    // This should point to index 1 which in turn points to 3+1=4
    assert_eq!(follow_address(1, IndirectA, 0, memory.len(), &memory), 4);
    assert_eq!(
        follow_address(1, PreDecrementIndirectA, 0, memory.len(), &memory),
        4
    );
    assert_eq!(
        follow_address(1, PostDecrementIndirectA, 0, memory.len(), &memory),
        4
    );

    // This should point to index 3 which points to itself (it's a DAT 0, 0)
    assert_eq!(follow_address(0, IndirectA, 3, memory.len(), &memory), 3);
    assert_eq!(
        follow_address(0, PreDecrementIndirectA, 3, memory.len(), &memory),
        3
    );
    assert_eq!(
        follow_address(0, PostDecrementIndirectA, 3, memory.len(), &memory),
        3
    );
}

// This tests that indirect b (including decrement) retrieves the correct index
#[test]
fn indirect_b() {
    let mut memory = generate_empty_memory(10);

    memory[0] = cmd! { DAT(None, 2, Immediate, 4, Immediate) };
    memory[1] = cmd! { DAT(None, 3, Immediate, 5, Immediate) };

    // Since memory is cyclic this should point to index 0 which in turn points to 4
    assert_eq!(follow_address(1, IndirectB, 9, memory.len(), &memory), 4);
    assert_eq!(
        follow_address(1, PreDecrementIndirectB, 9, memory.len(), &memory),
        4
    );
    assert_eq!(
        follow_address(1, PostDecrementIndirectB, 9, memory.len(), &memory),
        4
    );

    // This should point to index 1 which in turn points to 5+1=6
    assert_eq!(follow_address(1, IndirectB, 0, memory.len(), &memory), 6);
    assert_eq!(
        follow_address(1, PreDecrementIndirectB, 0, memory.len(), &memory),
        6
    );
    assert_eq!(
        follow_address(1, PostDecrementIndirectB, 0, memory.len(), &memory),
        6
    );

    // This should point to index 3 which points to itself (it's a DAT 0, 0)
    assert_eq!(follow_address(0, IndirectB, 3, memory.len(), &memory), 3);
    assert_eq!(
        follow_address(0, PreDecrementIndirectB, 3, memory.len(), &memory),
        3
    );
    assert_eq!(
        follow_address(0, PostDecrementIndirectB, 3, memory.len(), &memory),
        3
    );
}

#[test]
fn pre_decrement_a() {
    let looper = create_program! {
        // Skips the jump once the A (of the SEQ) is 0.
        SEQ(AB, 4, Immediate, 0, Immediate)
        // Decrement the A of the SEQ by 1 each time
        JMP(None, -1, Direct, -1, PreDecrementIndirectA)
    };

    let mut vm = VirtualMachine::new_simple(4, looper);

    for _ in 0..10 {
        vm.cycle();
    }

    assert_eq!(
        &vm.get_memory(),
        &(create_program! {
            SEQ(AB, 0, Immediate, 0, Immediate)
            JMP(None, -1, Direct, -1, PreDecrementIndirectA)
            DAT(None, 0, Immediate, 0, Immediate)
            DAT(None, 0, Immediate, 0, Immediate)
        })
        .as_slice()
    );

    assert_eq!(
        vm.get_users_pcs(),
        &[std::collections::VecDeque::from(vec![])]
    );
}

#[test]
fn pre_decrement_b() {
    let looper = create_program! {
        // Skips the jump once the B (of the SEQ) is 0.
        SEQ(AB, 0, Immediate, 4, Immediate)
        // Decrement the B of the SEQ by 1 each time
        JMP(None, -1, Direct, -1, PreDecrementIndirectB)
    };

    let mut vm = VirtualMachine::new_simple(4, looper);

    for _ in 0..10 {
        vm.cycle();
    }

    assert_eq!(
        &vm.get_memory(),
        &(create_program! {
            SEQ(AB, 0, Immediate, 0, Immediate)
            JMP(None, -1, Direct, -1, PreDecrementIndirectB)
            DAT(None, 0, Immediate, 0, Immediate)
            DAT(None, 0, Immediate, 0, Immediate)
        })
        .as_slice()
    );

    assert_eq!(
        vm.get_users_pcs(),
        &[std::collections::VecDeque::from(vec![])]
    );
}

#[test]
fn post_decrement_a() {
    let looper = create_program! {
        // Skips the jump once the A (of the SEQ) is 0.
        SEQ(AB, 4, Immediate, 0, Immediate)
        // Decrement the A of the SEQ by 1 each time
        JMP(None, -1, Direct, -1, PostDecrementIndirectA)
    };

    let mut vm = VirtualMachine::new_simple(4, looper);

    for _ in 0..10 {
        vm.cycle();
    }

    assert_eq!(
        &vm.get_memory(),
        &(create_program! {
            SEQ(AB, 0, Immediate, 0, Immediate)
            JMP(None, -1, Direct, -1, PostDecrementIndirectA)
            DAT(None, 0, Immediate, 0, Immediate)
            DAT(None, 0, Immediate, 0, Immediate)
        })
        .as_slice()
    );

    assert_eq!(
        vm.get_users_pcs(),
        &[std::collections::VecDeque::from(vec![])]
    );
}

#[test]
fn post_decrement_b() {
    let looper = create_program! {
        // Skips the jump once the B (of the SEQ) is 0.
        SEQ(AB, 0, Immediate, 4, Immediate)
        // Decrement the B of the SEQ by 1 each time
        JMP(None, -1, Direct, -1, PostDecrementIndirectB)
    };

    let mut vm = VirtualMachine::new_simple(4, looper);

    for _ in 0..10 {
        vm.cycle();
    }

    assert_eq!(
        &vm.get_memory(),
        &(create_program! {
            SEQ(AB, 0, Immediate, 0, Immediate)
            JMP(None, -1, Direct, -1, PostDecrementIndirectB)
            DAT(None, 0, Immediate, 0, Immediate)
            DAT(None, 0, Immediate, 0, Immediate)
        })
        .as_slice()
    );

    assert_eq!(
        vm.get_users_pcs(),
        &[std::collections::VecDeque::from(vec![])]
    );
}
