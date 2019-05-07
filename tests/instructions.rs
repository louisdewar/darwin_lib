use darwin_lib::{cmd, create_program, AddressMode, Instruction};

#[test]
fn create_single_instruction_with_macro() {
    // MOV instruction
    assert_eq!(
        cmd! { MOV(0, Direct, 1, Direct) },
        Instruction::MOV(0, AddressMode::Direct, 1, AddressMode::Direct)
    );

    // DAT instruction
    assert_eq!(
        cmd! { DAT(0, Direct, 1, Direct) },
        Instruction::DAT(0, AddressMode::Direct, 1, AddressMode::Direct)
    );

    // JMP instruction
    assert_eq!(
        cmd! { JMP(0, Direct) },
        Instruction::JMP(0, AddressMode::Direct)
    );

    // JMP with negative pointer instruction
    assert_eq!(
        cmd! { JMP(-1, Direct) },
        Instruction::JMP(-1, AddressMode::Direct)
    );

    // Ensure that DAT and MOV are different (as an example)
    assert_ne!(
        cmd! { DAT(0, Direct, 1, Direct) },
        Instruction::MOV(0, AddressMode::Direct, 1, AddressMode::Direct)
    );
}

#[test]
fn create_multiline_programs_with_macro() {
    assert_eq!(
        create_program! {
            MOV(0, Direct, 1, Direct)
            JMP(-1, Direct)
            DAT(0, Direct, 4, Direct)
        },
        vec![
            Instruction::MOV(0, AddressMode::Direct, 1, AddressMode::Direct),
            Instruction::JMP(-1, AddressMode::Direct),
            Instruction::DAT(0, AddressMode::Direct, 4, AddressMode::Direct),
        ]
    );
}
