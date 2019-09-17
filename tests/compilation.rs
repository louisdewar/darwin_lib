use darwin_lib::{create_program, parse_program};

#[test]
fn simple_program() {
    assert_eq!(
        parse_program("MOV 0 1").unwrap(),
        create_program! { MOV(I, 0, Direct, 1, Direct) },
    );
}

#[test]
fn single_value() {
    assert_eq!(
        parse_program("JMP 2").unwrap(),
        create_program! { JMP(B, 2, Direct, 0, Direct) },
    );
}
