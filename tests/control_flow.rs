use darwin_lib::{cmd, create_program, Instruction, VirtualMachine};

fn test_jmp(program: Vec<Instruction>, expected_location: usize) {
    use std::collections::VecDeque;

    let mut vm = VirtualMachine::new(20, vec![program]);

    vm.cycle();

    assert_eq!(
        vm.get_users_pcs(),
        &[VecDeque::from(vec![expected_location])]
    );
}

#[test]
fn spl() {
    let mut vm = VirtualMachine::new(
        20,
        vec![create_program! {
            SPL(None, 1, Direct, 0, Direct)
            JMP(None, -1, Direct, 0, Direct)
        }],
    );

    vm.cycle();

    assert_eq!(vm.get_users_pcs()[0].len(), 2,);

    // 16189 was found to be the time at which point there should be 8000 processes
    // 16500 should allow more SPL command to be executed so that if there are more than 8000 processes
    // a bug would be shown
    for _ in 0..16500 {
        vm.cycle();
    }

    assert_eq!(vm.get_users_pcs()[0].len(), 8000,);
}

#[test]
fn jmp() {
    // Standard
    test_jmp(
        create_program! {
            JMP(None, 3, Direct, 0, Direct)
        },
        3,
    );

    // IndirectB
    test_jmp(
        create_program! {
            JMP(None, 2, IndirectB, 0, Direct)
            DAT(None, 0, Direct, 3, Direct)
            DAT(None, 2, Direct, 5, Direct)
        },
        7,
    );

    // IndirectA
    test_jmp(
        create_program! {
            JMP(None, 2, IndirectA, 0, Direct)
            DAT(None, 0, Direct, 3, Direct)
            DAT(None, 2, Direct, 5, Direct)
        },
        4,
    );

    // Register b should do nothing
    test_jmp(
        create_program! {
            JMP(None, 3, Direct, 4, Direct)
        },
        3,
    );
}

#[test]
#[should_panic]
fn invalid_jmz() {
    // None is not a valid modifier
    test_jmp(
        create_program! {
            JMZ(None, 0, Direct, 0, Direct)
        },
        3,
    );
}

#[test]
fn jmz() {
    // Modifier: A, BA
    test_jmp(
        create_program! {
            JMZ(A, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 0, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMZ(BA, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 0, Direct)
        },
        1,
    );

    test_jmp(
        create_program! {
            JMZ(A, 3, Direct, 1, Direct) // Should jump
            DAT(None, 0, Direct, 2, Direct)
        },
        3,
    );

    test_jmp(
        create_program! {
            JMZ(BA, 3, Direct, 1, Direct) // Should jump
            DAT(None, 0, Direct, 2, Direct)
        },
        3,
    );

    // Modifier: B, AB
    test_jmp(
        create_program! {
            JMZ(B, 3, Direct, 1, Direct) // Should jump
            DAT(None, 1, Direct, 0, Direct)
        },
        3,
    );
    test_jmp(
        create_program! {
            JMZ(AB, 3, Direct, 1, Direct) // Should jump
            DAT(None, 1, Direct, 0, Direct)
        },
        3,
    );

    test_jmp(
        create_program! {
            JMZ(B, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 2, Direct)
        },
        1,
    );

    test_jmp(
        create_program! {
            JMZ(AB, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 2, Direct)
        },
        1,
    );

    // Modifier: F, X, I
    test_jmp(
        create_program! {
            JMZ(F, 3, Direct, 1, Direct) // Should jump
            DAT(None, 0, Direct, 0, Direct)
        },
        3,
    );
    test_jmp(
        create_program! {
            JMZ(X, 3, Direct, 1, Direct) // Should jump
            DAT(None, 0, Direct, 0, Direct)
        },
        3,
    );
    test_jmp(
        create_program! {
            JMZ(X, 3, Direct, 1, Direct) // Should jump
            DAT(None, 0, Direct, 0, Direct)
        },
        3,
    );

    test_jmp(
        create_program! {
            JMZ(F, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 1, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMZ(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 1, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMZ(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 1, Direct)
        },
        1,
    );

    test_jmp(
        create_program! {
            JMZ(F, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 1, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMZ(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 1, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMZ(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 1, Direct)
        },
        1,
    );

    test_jmp(
        create_program! {
            JMZ(F, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 0, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMZ(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 0, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMZ(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 0, Direct)
        },
        1,
    );
}

#[test]
#[should_panic]
fn invalid_jmn() {
    // None is not a valid modifier
    test_jmp(
        create_program! {
            JMN(None, 0, Direct, 0, Direct)
        },
        3,
    );
}

#[test]
fn jmn() {
    // Modifier: A, BA
    test_jmp(
        create_program! {
            JMN(A, 3, Direct, 1, Direct) // Should jump
            DAT(None, 1, Direct, 0, Direct)
        },
        3,
    );
    test_jmp(
        create_program! {
            JMN(BA, 3, Direct, 1, Direct) // Should jump
            DAT(None, 1, Direct, 0, Direct)
        },
        3,
    );

    test_jmp(
        create_program! {
            JMN(A, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 2, Direct)
        },
        1,
    );

    test_jmp(
        create_program! {
            JMN(BA, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 2, Direct)
        },
        1,
    );

    // Modifier: B, AB

    test_jmp(
        create_program! {
            JMN(B, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 0, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMN(AB, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 0, Direct)
        },
        1,
    );

    test_jmp(
        create_program! {
            JMN(B, 3, Direct, 1, Direct) // Should jump
            DAT(None, 0, Direct, 2, Direct)
        },
        3,
    );

    test_jmp(
        create_program! {
            JMN(AB, 3, Direct, 1, Direct) // Should jump
            DAT(None, 0, Direct, 2, Direct)
        },
        3,
    );

    // Modifier: F, X, I
    test_jmp(
        create_program! {
            JMN(F, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 0, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMN(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 0, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMN(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 0, Direct)
        },
        1,
    );

    test_jmp(
        create_program! {
            JMN(F, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 1, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMN(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 1, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMN(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 0, Direct, 1, Direct)
        },
        1,
    );

    test_jmp(
        create_program! {
            JMN(F, 3, Direct, 1, Direct) // Should jump
            DAT(None, 1, Direct, 1, Direct)
        },
        3,
    );
    test_jmp(
        create_program! {
            JMN(X, 3, Direct, 1, Direct) // Should jump
            DAT(None, 1, Direct, 1, Direct)
        },
        3,
    );
    test_jmp(
        create_program! {
            JMN(X, 3, Direct, 1, Direct) // Should jump
            DAT(None, 1, Direct, 1, Direct)
        },
        3,
    );

    test_jmp(
        create_program! {
            JMN(F, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 0, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMN(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 0, Direct)
        },
        1,
    );
    test_jmp(
        create_program! {
            JMN(X, 3, Direct, 1, Direct) // Shouldn't jump
            DAT(None, 1, Direct, 0, Direct)
        },
        1,
    );
}

#[test]
fn djn() {
    use std::collections::VecDeque;

    fn test_djn(program: Vec<Instruction>, expected_2: Instruction, expected_3: Instruction) {
        let mut vm = VirtualMachine::new(20, vec![program]);

        for _ in 0..8 {
            vm.cycle();
        }

        assert_eq!(vm.get_users_pcs(), &[VecDeque::from(vec![2])]);

        assert_eq!(vm.get_memory()[2], expected_2);

        assert_eq!(vm.get_memory()[3], expected_3);
    }

    // Modifier: A, AB
    test_djn(
        create_program! {
            ADD(A, 1, Immediate, 3, Direct)
            DJN(A, -1, Direct, 1, Direct)
            DAT(None, 4, Direct, 4, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        cmd! { DAT(None, 0, Direct, 4, Direct) },
        cmd! { DAT(None, 4, Direct, 0, Direct) },
    );
    test_djn(
        create_program! {
            ADD(A, 1, Immediate, 3, Direct)
            DJN(BA, -1, Direct, 1, Direct)
            DAT(None, 4, Direct, 4, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        cmd! { DAT(None, 0, Direct, 4, Direct) },
        cmd! { DAT(None, 4, Direct, 0, Direct) },
    );

    // Modifier: B, BA
    test_djn(
        create_program! {
            ADD(A, 1, Immediate, 3, Direct)
            DJN(B, -1, Direct, 1, Direct)
            DAT(None, 4, Direct, 4, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        cmd! { DAT(None, 4, Direct, 0, Direct) },
        cmd! { DAT(None, 4, Direct, 0, Direct) },
    );
    test_djn(
        create_program! {
            ADD(A, 1, Immediate, 3, Direct)
            DJN(AB, -1, Direct, 1, Direct)
            DAT(None, 4, Direct, 4, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        cmd! { DAT(None, 4, Direct, 0, Direct) },
        cmd! { DAT(None, 4, Direct, 0, Direct) },
    );

    // Modifier: F, X, I
    test_djn(
        create_program! {
            ADD(A, 1, Immediate, 3, Direct)
            DJN(F, -1, Direct, 1, Direct)
            DAT(None, 4, Direct, 4, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        cmd! { DAT(None, 0, Direct, 0, Direct) },
        cmd! { DAT(None, 4, Direct, 0, Direct) },
    );
    test_djn(
        create_program! {
            ADD(A, 1, Immediate, 3, Direct)
            DJN(X, -1, Direct, 1, Direct)
            DAT(None, 4, Direct, 4, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        cmd! { DAT(None, 0, Direct, 0, Direct) },
        cmd! { DAT(None, 4, Direct, 0, Direct) },
    );
    test_djn(
        create_program! {
            ADD(A, 1, Immediate, 3, Direct)
            DJN(I, -1, Direct, 1, Direct)
            DAT(None, 4, Direct, 4, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        cmd! { DAT(None, 0, Direct, 0, Direct) },
        cmd! { DAT(None, 4, Direct, 0, Direct) },
    );
}
