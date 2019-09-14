use darwin_lib::{VirtualMachine, Instruction, create_program, cmd};

fn test_operation(program: Vec<Instruction>, result_mem_location: usize, result: Instruction) {
    let mut vm = VirtualMachine::new(20, vec![program]);
    vm.cycle();

    assert_eq!(
        vm.get_memory()[result_mem_location],
        result
    )
}

#[test]
fn test_add() {
    // Modifier: A
    test_operation(
        create_program!(
            ADD(A, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 1, Immediate)
            DAT(None, 2, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 3, Immediate, 1, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            ADD(A, 1, Direct, 2, Direct)
            DAT(None, 15, Immediate, 1, Immediate)
            DAT(None, 15, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 10, Immediate, 1, Immediate) }
    );
    // Modifier: B
    test_operation(
        create_program!(
            ADD(B, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 1, Immediate)
            DAT(None, 2, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 2, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            ADD(B, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 15, Immediate)
            DAT(None, 2, Immediate, 15, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 10, Immediate) }
    );
    // Modifier: AB
    test_operation(
        create_program!(
            ADD(AB, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 1, Immediate)
            DAT(None, 2, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 4, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            ADD(AB, 1, Direct, 2, Direct)
            DAT(None, 15, Immediate, 1, Immediate)
            DAT(None, 2, Immediate, 15, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 10, Immediate) }
    );
    // Modifier: BA
    test_operation(
        create_program!(
            ADD(BA, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 1, Immediate)
            DAT(None, 2, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 3, Immediate, 1, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            ADD(BA, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 15, Immediate)
            DAT(None, 15, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 10, Immediate, 1, Immediate) }
    );
    // Modifier: X
    test_operation(
        create_program!(
            ADD(X, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 2, Immediate)
            DAT(None, 3, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 5, Immediate, 3, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            ADD(X, 1, Direct, 2, Direct)
            DAT(None, 14, Immediate, 15, Immediate)
            DAT(None, 17, Immediate, 16, Immediate)
        ),
        2,
        cmd!{ DAT(None, 12, Immediate, 10, Immediate) }
    );
    // Modifier: F
    test_operation(
        create_program!(
            ADD(F, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 4, Immediate)
            DAT(None, 1, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 4, Immediate, 6, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            ADD(F, 1, Direct, 2, Direct)
            DAT(None, 15, Immediate, 15, Immediate)
            DAT(None, 15, Immediate, 15, Immediate)
        ),
        2,
        cmd!{ DAT(None, 10, Immediate, 10, Immediate) }
    );
    // Modifier: I
    test_operation(
        create_program!(
            ADD(I, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 4, Immediate)
            DAT(None, 1, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 4, Immediate, 6, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            ADD(I, 1, Direct, 2, Direct)
            DAT(None, 15, Immediate, 15, Immediate)
            DAT(None, 15, Immediate, 15, Immediate)
        ),
        2,
        cmd!{ DAT(None, 10, Immediate, 10, Immediate) }
    );
    // Modifier: None
    test_operation(
        create_program!(
            ADD(None, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 1, Immediate)
            DAT(None, 2, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 4, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            ADD(None, 1, Direct, 2, Direct)
            DAT(None, 15, Immediate, 1, Immediate)
            DAT(None, 2, Immediate, 15, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 10, Immediate) }
    );
}

#[test]
fn test_sub() {
    // Modifier: A
    test_operation(
        create_program!(
            SUB(A, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 1, Immediate)
            DAT(None, 4, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 3, Immediate, 1, Immediate) }
    );
    // Resulting number negative
    test_operation(
        create_program!(
            SUB(A, 1, Direct, 2, Direct)
            DAT(None, 4, Immediate, 1, Immediate)
            DAT(None, 1, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 17, Immediate, 1, Immediate) }
    );
    // Modifier: B
    test_operation(
        create_program!(
            SUB(B, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 1, Immediate)
            DAT(None, 1, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 1, Immediate) }
    );
    // Resulting number negative
    test_operation(
        create_program!(
            SUB(B, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 4, Immediate)
            DAT(None, 1, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 17, Immediate) }
    );
    // Modifier: AB
    test_operation(
        create_program!(
            SUB(AB, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 1, Immediate)
            DAT(None, 1, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 1, Immediate) }
    );
    // Resulting number negative
    test_operation(
        create_program!(
            SUB(AB, 1, Direct, 2, Direct)
            DAT(None, 4, Immediate, 1, Immediate)
            DAT(None, 1, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 17, Immediate) }
    );
    // Modifier: BA
    test_operation(
        create_program!(
            SUB(BA, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 1, Immediate)
            DAT(None, 2, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 1, Immediate) }
    );
    // Resulting number negative
    test_operation(
        create_program!(
            SUB(BA, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 4, Immediate)
            DAT(None, 1, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 17, Immediate, 1, Immediate) }
    );
    // Modifier: X
    test_operation(
        create_program!(
            SUB(X, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 2, Immediate)
            DAT(None, 3, Immediate, 4, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 3, Immediate) }
    );
    // Resulting number negative
    test_operation(
        create_program!(
            SUB(X, 1, Direct, 2, Direct)
            DAT(None, 4, Immediate, 3, Immediate)
            DAT(None, 1, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 18, Immediate, 17, Immediate) }
    );
    // Modifier: F
    test_operation(
        create_program!(
            SUB(F, 1, Direct, 2, Direct)
            DAT(None, 2, Immediate, 1, Immediate)
            DAT(None, 3, Immediate, 4, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 3, Immediate) }
    );
    // Resulting number negative
    test_operation(
        create_program!(
            SUB(F, 1, Direct, 2, Direct)
            DAT(None, 4, Immediate, 3, Immediate)
            DAT(None, 1, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 17, Immediate, 18, Immediate) }
    );
    // Modifier: I
    test_operation(
        create_program!(
            SUB(I, 1, Direct, 2, Direct)
            DAT(None, 2, Immediate, 1, Immediate)
            DAT(None, 3, Immediate, 4, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 3, Immediate) }
    );
    // Resulting number negative
    test_operation(
        create_program!(
            SUB(I, 1, Direct, 2, Direct)
            DAT(None, 4, Immediate, 3, Immediate)
            DAT(None, 1, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 17, Immediate, 18, Immediate) }
    );
    // Modifier: None
    test_operation(
        create_program!(
            SUB(None, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 1, Immediate)
            DAT(None, 1, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 1, Immediate) }
    );
    // Resulting number negative
    test_operation(
        create_program!(
            SUB(None, 1, Direct, 2, Direct)
            DAT(None, 4, Immediate, 1, Immediate)
            DAT(None, 1, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 17, Immediate) }
    );
}

#[test]
fn test_mul() {
    // Modifier: A
    test_operation(
        create_program!(
            MUL(A, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 2, Immediate)
            DAT(None, 2, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 6, Immediate, 2, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            MUL(A, 1, Direct, 2, Direct)
            DAT(None, 5, Immediate, 2, Immediate)
            DAT(None, 6, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 10, Immediate, 2, Immediate) }
    );
    // Modifier: B
    test_operation(
        create_program!(
            MUL(B, 1, Direct, 2, Direct)
            DAT(None, 2, Immediate, 2, Immediate)
            DAT(None, 2, Immediate, 3, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 6, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            MUL(B, 1, Direct, 2, Direct)
            DAT(None, 2, Immediate, 5, Immediate)
            DAT(None, 2, Immediate, 6, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 10, Immediate) }
    );
    // Modifier: AB
    test_operation(
        create_program!(
            MUL(AB, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 2, Immediate)
            DAT(None, 2, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 6, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            MUL(AB, 1, Direct, 2, Direct)
            DAT(None, 5, Immediate, 2, Immediate)
            DAT(None, 2, Immediate, 6, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 10, Immediate) }
    );
    // Modifier: BA
    test_operation(
        create_program!(
            MUL(BA, 1, Direct, 2, Direct)
            DAT(None, 2, Immediate, 3, Immediate)
            DAT(None, 2, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 6, Immediate, 2, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            MUL(BA, 1, Direct, 2, Direct)
            DAT(None, 2, Immediate, 5, Immediate)
            DAT(None, 6, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 10, Immediate, 2, Immediate) }
    );
    // Modifier: X
    test_operation(
        create_program!(
            MUL(X, 1, Direct, 2, Direct)
            DAT(None, 2, Immediate, 3, Immediate)
            DAT(None, 4, Immediate, 3, Immediate)
        ),
        2,
        cmd!{ DAT(None, 12, Immediate, 6, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            MUL(X, 1, Direct, 2, Direct)
            DAT(None, 6, Immediate, 6, Immediate)
            DAT(None, 5, Immediate, 5, Immediate)
        ),
        2,
        cmd!{ DAT(None, 10, Immediate, 10, Immediate) }
    );
    // Modifier: F
    test_operation(
        create_program!(
            MUL(F, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 4, Immediate)
            DAT(None, 2, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 6, Immediate, 8, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            MUL(F, 1, Direct, 2, Direct)
            DAT(None, 5, Immediate, 5, Immediate)
            DAT(None, 6, Immediate, 6, Immediate)
        ),
        2,
        cmd!{ DAT(None, 10, Immediate, 10, Immediate) }
    );
    // Modifier: I
    test_operation(
        create_program!(
            MUL(I, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 4, Immediate)
            DAT(None, 2, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 6, Immediate, 8, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            MUL(I, 1, Direct, 2, Direct)
            DAT(None, 5, Immediate, 5, Immediate)
            DAT(None, 6, Immediate, 6, Immediate)
        ),
        2,
        cmd!{ DAT(None, 10, Immediate, 10, Immediate) }
    );
    // Modifier: None
    test_operation(
        create_program!(
            MUL(None, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 2, Immediate)
            DAT(None, 2, Immediate, 2, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 6, Immediate) }
    );
    // Resulting number greater than memory size
    test_operation(
        create_program!(
            MUL(None, 1, Direct, 2, Direct)
            DAT(None, 6, Immediate, 2, Immediate)
            DAT(None, 2, Immediate, 5, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 10, Immediate) }
    );
}

#[test]
fn test_div() {
    // Should ignore decimal
    test_operation(
        create_program!(
            DIV(A, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 1, Immediate)
            DAT(None, 10, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 3, Immediate, 1, Immediate) }
    );
    // Modifier: A
    test_operation(
        create_program!(
            DIV(A, 1, Direct, 2, Direct)
            DAT(None, 5, Immediate, 1, Immediate)
            DAT(None, 10, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 1, Immediate) }
    );
    // Modifier: B
    test_operation(
        create_program!(
            DIV(B, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 5, Immediate)
            DAT(None, 1, Immediate, 10, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 2, Immediate) }
    );
    // Modifier: AB
    test_operation(
        create_program!(
            DIV(AB, 1, Direct, 2, Direct)
            DAT(None, 5, Immediate, 1, Immediate)
            DAT(None, 1, Immediate, 10, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 2, Immediate) }
    );
    // Modifier: BA
    test_operation(
        create_program!(
            DIV(BA, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 5, Immediate)
            DAT(None, 10, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 1, Immediate) }
    );
    // Modifier: X
    test_operation(
        create_program!(
            DIV(X, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 5, Immediate)
            DAT(None, 10, Immediate, 9, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 3, Immediate) }
    );
    // Modifier: F
    test_operation(
        create_program!(
            DIV(F, 1, Direct, 2, Direct)
            DAT(None, 5, Immediate, 3, Immediate)
            DAT(None, 10, Immediate, 9, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 3, Immediate) }
    );
    // Modifier: I
    test_operation(
        create_program!(
            DIV(I, 1, Direct, 2, Direct)
            DAT(None, 5, Immediate, 3, Immediate)
            DAT(None, 10, Immediate, 9, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 3, Immediate) }
    );
    // Modifier: None
    test_operation(
        create_program!(
            DIV(None, 1, Direct, 2, Direct)
            DAT(None, 5, Immediate, 1, Immediate)
            DAT(None, 1, Immediate, 10, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 2, Immediate) }
    );
}

#[test]
fn div_by_zero() {
    let mut vm = VirtualMachine::new(
        20,
        vec![
            create_program!(
                DIV(F, 1, Direct, 2, Direct)
                DAT(None, 0, Immediate, 2, Immediate)
                DAT(None, 1, Immediate, 10, Immediate)
            )
        ]
    );
    vm.cycle();
    // Valid b_reg division should still be performed
    assert_eq!(
        vm.get_memory()[2],
        cmd!{ DAT(None, 1, Immediate, 5, Immediate) }
    );
    // Current process should be terminated
    assert_eq!(
        vm.get_users_pcs(),
        [[]]
    );
}

#[test]
fn test_mod() {
    // Modifier: A
    test_operation(
        create_program!(
            MOD(A, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 1, Immediate)
            DAT(None, 8, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 1, Immediate) }
    );
    test_operation(
        create_program!(
            MOD(A, 1, Direct, 2, Direct)
            DAT(None, 8, Immediate, 1, Immediate)
            DAT(None, 3, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 3, Immediate, 1, Immediate) }
    );
    // Modifier: B
    test_operation(
        create_program!(
            MOD(B, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 3, Immediate)
            DAT(None, 1, Immediate, 10, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 1, Immediate) }
    );
    // Modifier: AB
    test_operation(
        create_program!(
            MOD(AB, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 1, Immediate)
            DAT(None, 1, Immediate, 8, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 2, Immediate) }
    );
    // Modifier: BA
    test_operation(
        create_program!(
            MOD(BA, 1, Direct, 2, Direct)
            DAT(None, 1, Immediate, 3, Immediate)
            DAT(None, 8, Immediate, 1, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 1, Immediate) }
    );
    // Modifier: X
    test_operation(
        create_program!(
            MOD(X, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 3, Immediate)
            DAT(None, 8, Immediate, 9, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 0, Immediate) }
    );
    // Modifier: F
    test_operation(
        create_program!(
            MOD(F, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 3, Immediate)
            DAT(None, 8, Immediate, 9, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 0, Immediate) }
    );
    // Modifier: I
    test_operation(
        create_program!(
            MOD(I, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 3, Immediate)
            DAT(None, 8, Immediate, 9, Immediate)
        ),
        2,
        cmd!{ DAT(None, 2, Immediate, 0, Immediate) }
    );
    // Modifier: None
    test_operation(
        create_program!(
            MOD(None, 1, Direct, 2, Direct)
            DAT(None, 3, Immediate, 1, Immediate)
            DAT(None, 1, Immediate, 8, Immediate)
        ),
        2,
        cmd!{ DAT(None, 1, Immediate, 2, Immediate) }
    );
}
