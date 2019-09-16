use darwin_lib::{create_program, VirtualMachine};

fn main() {
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

    for (i, instruction) in vm.get_memory().iter().enumerate() {
        println!("{:02}. {}", i, instruction);
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
