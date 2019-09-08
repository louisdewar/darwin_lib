use darwin_lib::{cmd, create_program, AddressMode, Instruction, OpCode, Modifier};

#[test]
fn create_single_instruction_with_macro() {
    // MOV instruction
    assert_eq!(
        cmd! { MOV(I, 0, Direct, 1, Direct) },
        Instruction::new(OpCode::MOV, Modifier::I, 0, AddressMode::Direct, 1, AddressMode::Direct)
    );

    // DAT instruction
    assert_eq!(
        cmd! { DAT(None, 0, Direct, 1, Direct) },
        Instruction::new(OpCode::DAT, Modifier::None, 0, AddressMode::Direct, 1, AddressMode::Direct)
    );

    // JMP instruction
    assert_eq!(
        cmd! { JMP(None, 0, Direct) },
        Instruction::new(OpCode::JMP, Modifier::None, 0, AddressMode::Direct, 0, AddressMode::Direct)
    );

    // JMP with negative pointer instruction
    assert_eq!(
        cmd! { JMP(None, -1, Direct) },
        Instruction::new(OpCode::JMP, Modifier::None, -1, AddressMode::Direct, 0, AddressMode::Direct)
    );

    // Ensure that DAT and MOV are different (as an example)
    assert_ne!(
        cmd! { DAT(None, 0, Direct, 1, Direct) },
        Instruction::new(OpCode::MOV, Modifier::None, 0, AddressMode::Direct, 1, AddressMode::Direct)
    );
}

#[test]
fn create_multiline_programs_with_macro() {
    assert_eq!(
        create_program! {
            MOV(I, 0, Direct, 1, Direct)
            JMP(None, -1, Direct)
            DAT(None, 0, Direct, 4, Direct)
        },
        vec![
            Instruction::new(OpCode::MOV, Modifier::I, 0, AddressMode::Direct, 1, AddressMode::Direct),
            Instruction::new(OpCode::JMP, Modifier::None, -1, AddressMode::Direct, 0, AddressMode::Direct),
            Instruction::new(OpCode::DAT, Modifier::None, 0, AddressMode::Direct, 4, AddressMode::Direct),
        ]
    );
}
