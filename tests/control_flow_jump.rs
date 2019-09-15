use darwin_lib::{cmd, create_program, Instruction, VirtualMachine};

fn test_jmp(program: Vec<Instruction>, expected_location: usize) {
    use std::collections::VecDeque;

    let mut vm = VirtualMachine::new_simple(20, program);

    vm.cycle();

    assert_eq!(
        vm.get_users_pcs(),
        &[VecDeque::from(vec![expected_location])]
    );
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

// Assumes that the instruction will jump to location 3
fn use_test_matrix(instruction: Instruction, matrix: &[Instruction], results: &[bool]) {
    for (dat, should_jump) in matrix.iter().zip(results.iter()) {
        test_jmp(vec![instruction, *dat], if *should_jump { 3 } else { 1 });
    }
}

#[test]
fn jmz() {
    let test_matrix = [
        cmd! { DAT(None, 0, Direct, 0, Direct) },
        cmd! { DAT(None, 1, Direct, 0, Direct) },
        cmd! { DAT(None, 0, Direct, 1, Direct) },
        cmd! { DAT(None, 1, Direct, 1, Direct) },
    ];

    use_test_matrix(
        cmd! { JMZ(A, 3, Direct, 1, Direct) },
        &test_matrix,
        &[true, false, true, false],
    );

    use_test_matrix(
        cmd! { JMZ(BA, 3, Direct, 1, Direct) },
        &test_matrix,
        &[true, false, true, false],
    );

    use_test_matrix(
        cmd! { JMZ(B, 3, Direct, 1, Direct) },
        &test_matrix,
        &[true, true, false, false],
    );

    use_test_matrix(
        cmd! { JMZ(AB, 3, Direct, 1, Direct) },
        &test_matrix,
        &[true, true, false, false],
    );

    use_test_matrix(
        cmd! { JMZ(F, 3, Direct, 1, Direct) },
        &test_matrix,
        &[true, false, false, false],
    );

    use_test_matrix(
        cmd! { JMZ(X, 3, Direct, 1, Direct) },
        &test_matrix,
        &[true, false, false, false],
    );

    use_test_matrix(
        cmd! { JMZ(I, 3, Direct, 1, Direct) },
        &test_matrix,
        &[true, false, false, false],
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
    let test_matrix = [
        cmd! { DAT(None, 0, Direct, 0, Direct) },
        cmd! { DAT(None, 1, Direct, 0, Direct) },
        cmd! { DAT(None, 0, Direct, 1, Direct) },
        cmd! { DAT(None, 1, Direct, 1, Direct) },
    ];

    use_test_matrix(
        cmd! { JMN(A, 3, Direct, 1, Direct) },
        &test_matrix,
        &[false, true, false, true],
    );

    use_test_matrix(
        cmd! { JMN(BA, 3, Direct, 1, Direct) },
        &test_matrix,
        &[false, true, false, true],
    );

    use_test_matrix(
        cmd! { JMN(B, 3, Direct, 1, Direct) },
        &test_matrix,
        &[false, false, true, true],
    );

    use_test_matrix(
        cmd! { JMN(AB, 3, Direct, 1, Direct) },
        &test_matrix,
        &[false, false, true, true],
    );

    use_test_matrix(
        cmd! { JMN(F, 3, Direct, 1, Direct) },
        &test_matrix,
        &[false, false, false, true],
    );

    use_test_matrix(
        cmd! { JMN(X, 3, Direct, 1, Direct) },
        &test_matrix,
        &[false, false, false, true],
    );

    use_test_matrix(
        cmd! { JMN(I, 3, Direct, 1, Direct) },
        &test_matrix,
        &[false, false, false, true],
    );
}

#[test]
#[should_panic]
fn invalid_djn() {
    // None is not a valid modifier
    test_jmp(
        create_program! {
            DJN(None, 3, Direct, 0, Direct)
        },
        3,
    );
}

#[test]
fn djn() {
    use std::collections::VecDeque;

    fn test_djn(program: Vec<Instruction>, expected_2: Instruction, expected_3: Instruction) {
        let mut vm = VirtualMachine::new_simple(20, program);

        for _ in 0..8 {
            vm.cycle();
        }

        assert_eq!(vm.get_users_pcs(), &[VecDeque::from(vec![2])]);

        assert_eq!(vm.get_memory()[2], expected_2);

        assert_eq!(vm.get_memory()[3], expected_3);
    }

    // Modifier: A, BA
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

    // Modifier: B, AB
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
