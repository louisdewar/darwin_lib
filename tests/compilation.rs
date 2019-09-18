use darwin_lib::{create_program, parse_program, ParseError};

#[test]
fn simple_program() {
    assert_eq!(
        parse_program("MOV 0 1\nDAT #2 $3").unwrap(),
        create_program! {
            MOV(I, 0, Direct, 1, Direct)
            DAT(F, 2, Immediate, 3, Direct)
        },
    );
}

#[test]
fn every_opcode() {
    // MOV
    assert_eq!(
        parse_program("MOV.I 0 1").unwrap(),
        create_program! { MOV(I, 0, Direct, 1, Direct) },
    );
    // ADD
    assert_eq!(
        parse_program("ADD.BA #3 *1").unwrap(),
        create_program! { ADD(BA, 3, Immediate, 1, IndirectA) },
    );
    // SUB
    assert_eq!(
        parse_program("SUB.AB #3 @1").unwrap(),
        create_program! { SUB(AB, 3, Immediate, 1, IndirectB) },
    );
    // MUL
    assert_eq!(
        parse_program("MUL.F 1 2").unwrap(),
        create_program! { MUL(F, 1, Direct, 2, Direct) },
    );
    // DIV
    assert_eq!(
        parse_program("DIV.X 1 2").unwrap(),
        create_program! { DIV(X, 1, Direct, 2, Direct) },
    );
    // MOD
    assert_eq!(
        parse_program("MOD.A 1 2").unwrap(),
        create_program! { MOD(A, 1, Direct, 2, Direct) },
    );
    // DAT
    assert_eq!(
        parse_program("DAT 0 0").unwrap(),
        create_program! { DAT(F, 0, Direct, 0, Direct) },
    );
    // JMP
    assert_eq!(
        parse_program("JMP 2").unwrap(),
        create_program! { JMP(B, 2, Direct, 0, Direct) },
    );
    // SPL
    assert_eq!(
        parse_program("SPL 2").unwrap(),
        create_program! { SPL(B, 2, Direct, 0, Direct) },
    );
    // JMZ
    assert_eq!(
        parse_program("JMZ 2 #0").unwrap(),
        create_program! { JMZ(B, 2, Direct, 0, Immediate) },
    );
    // JMN
    assert_eq!(
        parse_program("JMN 2 #1").unwrap(),
        create_program! { JMN(B, 2, Direct, 1, Immediate) },
    );
    // NOP
    assert_eq!(
        parse_program("NOP 0 0").unwrap(),
        create_program! { NOP(F, 0, Direct, 0, Direct) },
    );
    // DJN
    assert_eq!(
        parse_program("DJN 2 #2").unwrap(),
        create_program! { DJN(B, 2, Direct, 2, Immediate) },
    );
    // SEQ
    assert_eq!(
        parse_program("SEQ.F 1 2").unwrap(),
        create_program! { SEQ(F, 1, Direct, 2, Direct) },
    );
    // SNE
    assert_eq!(
        parse_program("SNE.A 1 2").unwrap(),
        create_program! { SNE(A, 1, Direct, 2, Direct) },
    );
    // SLT
    assert_eq!(
        parse_program("SLT.B 1 2").unwrap(),
        create_program! { SLT(B, 1, Direct, 2, Direct) },
    );
}

#[test]
fn invalid_modifier() {
    assert_eq!(
        match parse_program("DAT.I 0 0") {
            Ok(_) => panic!("An error should have occured, but didn't"),
            Err(e) => e,
        },
        ParseError::InvalidModifier(1)
    )
}

#[test]
fn not_enough_arguments() {
    assert_eq!(
        match parse_program("MOV 0") {
            Ok(_) => panic!("An error should have occured, but didn't"),
            Err(e) => e,
        },
        ParseError::NotEnoughArgumets(1)
    )
}

#[test]
fn unexpected_argument() {
    assert_eq!(
        match parse_program("MOV 0 0 0") {
            Ok(_) => panic!("An error should have occured, but didn't"),
            Err(e) => e,
        },
        ParseError::UnexpectedArgument(1)
    )
}

#[test]
fn unknown_opcode() {
    assert_eq!(
        match parse_program("ABC 0 0") {
            Ok(_) => panic!("An error should have occured, but didn't"),
            Err(e) => e,
        },
        ParseError::UnknownOpCode((1, "ABC"))
    )
}

#[test]
fn unknown_modifier() {
    assert_eq!(
        match parse_program("DAT.XYZ 0 0") {
            Ok(_) => panic!("An error should have occured, but didn't"),
            Err(e) => e,
        },
        ParseError::UnknownModifier((1, "XYZ"))
    )
}

#[test]
fn unknown_value() {
    assert_eq!(
        match parse_program("DAT zero 0") {
            Ok(_) => panic!("An error should have occured, but didn't"),
            Err(e) => e,
        },
        ParseError::UnknownValue((1, "zero"))
    )
}
