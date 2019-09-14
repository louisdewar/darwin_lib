use darwin_lib::{create_program, Instruction, VirtualMachine};

fn test_seq(program: Vec<Instruction>, should_skip: bool) {
    use std::collections::VecDeque;

    let mut vm = VirtualMachine::new(20, vec![program]);

    vm.cycle();

    if should_skip {
        assert_eq!(
            vm.get_users_pcs(),
            &[VecDeque::from(vec![2])],
            "Did not skip when it should have"
        )
    } else {
        assert_eq!(
            vm.get_users_pcs(),
            &[VecDeque::from(vec![1])],
            "Skipped when it shouldn't have"
        )
    }
}

#[test]
#[should_panic]
fn invalid_seq() {
    // None is not a valid modifier
    test_seq(
        create_program! {
            SEQ(None, 0, Direct, 0, Direct)
        },
        false,
    );
}

#[test]
fn seq() {
    // Modifier: A
    test_seq(
        create_program! {
            SEQ(A, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(A, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 1, Direct, 1, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(A, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );

    // Modifier: BA
    test_seq(
        create_program! {
            SEQ(BA, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );

    test_seq(
        create_program! {
            SEQ(BA, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        true,
    );

    test_seq(
        create_program! {
            SEQ(BA, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );

    // Modifier: B
    test_seq(
        create_program! {
            SEQ(B, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(B, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 1, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(B, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );

    // Modifier: AB
    test_seq(
        create_program! {
            SEQ(AB, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(AB, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(AB, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );

    // Modifier: F, X, I
    test_seq(
        create_program! {
            SEQ(F, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(X, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(I, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );

    test_seq(
        create_program! {
            SEQ(F, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SEQ(X, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );

    test_seq(
        create_program! {
            SEQ(F, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 1, Direct)
            MOV(None, 1, Direct, 1, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(X, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 1, Direct)
            MOV(None, 1, Direct, 1, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(I, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 1, Direct)
            MOV(None, 1, Direct, 1, Direct)
        },
        false,
    );

    test_seq(
        create_program! {
            SEQ(F, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 1, Direct)
            DAT(A, 1, Direct, 1, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(X, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 1, Direct)
            DAT(A, 1, Direct, 1, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SEQ(I, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 1, Direct)
            DAT(A, 1, Direct, 1, Direct)
        },
        false,
    );

    test_seq(
        create_program! {
            SEQ(F, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 1, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        false,
    );

    test_seq(
        create_program! {
            SEQ(F, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SEQ(I, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SEQ(X, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        true,
    );

    test_seq(
        create_program! {
            SEQ(F, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SEQ(I, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SEQ(X, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        true,
    );
}

#[test]
#[should_panic]
fn invalid_sne() {
    // None is not a valid modifier
    test_seq(
        create_program! {
            SNE(None, 0, Direct, 0, Direct)
        },
        false,
    );
}

#[test]
fn sne() {
    // Modifier: A
    test_seq(
        create_program! {
            SNE(A, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(A, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(A, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(A, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SNE(A, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        true,
    );

    // Modifier: AB
    test_seq(
        create_program! {
            SNE(AB, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(AB, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(AB, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(AB, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SNE(AB, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        true,
    );

    // Modifier: BA
    test_seq(
        create_program! {
            SNE(BA, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(BA, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(BA, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(BA, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SNE(BA, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        true,
    );

    // Modifier: B
    test_seq(
        create_program! {
            SNE(B, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(B, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(B, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SNE(B, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SNE(B, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        true,
    );

    // Modifier: F, X, I (Should all be indentical)
    test_seq(
        create_program! {
            SNE(F, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SNE(X, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SNE(I, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );

    test_seq(
        create_program! {
            SNE(F, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SNE(X, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );
    test_seq(
        create_program! {
            SNE(I, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );

    test_seq(
        create_program! {
            SNE(F, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        true,
    );

    test_seq(
        create_program! {
            SNE(F, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        true,
    );

    test_seq(
        create_program! {
            SNE(F, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        true,
    );

    test_seq(
        create_program! {
            SNE(F, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
}

#[test]
#[should_panic]
fn invalid_slt_none() {
    // None is not a valid modifier
    test_seq(
        create_program! {
            SLT(None, 0, Direct, 0, Direct)
        },
        false,
    );
}

#[test]
#[should_panic]
fn invalid_slt_i() {
    // I is not a valid modifier
    test_seq(
        create_program! {
            SLT(I, 0, Direct, 0, Direct)
        },
        false,
    );
}

#[test]
fn slt() {
    // Modifier: A
    test_seq(
        create_program! {
            SLT(A, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SLT(A, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SLT(A, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        true,
    );

    // Modifier: B
    test_seq(
        create_program! {
            SLT(B, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SLT(B, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SLT(B, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        true,
    );

    // Modifier: AB
    test_seq(
        create_program! {
            SLT(AB, 1, Direct, 2, Direct)
            DAT(None, 1, Direct, 0, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SLT(AB, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SLT(AB, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        true,
    );

    // Modifier: BA
    test_seq(
        create_program! {
            SLT(BA, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 1, Direct)
            DAT(None, 0, Direct, 0, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SLT(BA, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 0, Direct, 1, Direct)
        },
        false,
    );
    test_seq(
        create_program! {
            SLT(BA, 1, Direct, 2, Direct)
            DAT(None, 0, Direct, 0, Direct)
            DAT(None, 1, Direct, 0, Direct)
        },
        true,
    );
}
