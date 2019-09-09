/// Creates a single instruction
/// # Example
/// ```
/// use darwin_lib::{ cmd, Instruction, AddressMode as am, OpCode as op, Modifier as m };
/// // MOV instruction
/// assert_eq!(cmd! { MOV(I, 0, Direct, 1, Direct) }, Instruction::new(op::MOV, m::I, 0, am::Direct, 1, am::Direct));
///
/// // DAT instruction
/// assert_eq!(cmd! { DAT(None, 0, Direct, 1, Direct) }, Instruction::new(op::DAT, m::None, 0, am::Direct, 1, am::Direct));
///
/// // JMP instruction
/// assert_eq!(cmd! { JMP(None, 0, Direct) }, Instruction::new(op::JMP, m::None, 0, am::Direct, 0, am::Direct));
/// ```
#[macro_export]
macro_rules! cmd {
    // Single parameter
    ($op_code:tt ($modifier:tt, $reg_a:expr, $mode_a:tt)) => {
        // B field is set to 0 direct
        $crate::Instruction::new(
            $crate::OpCode::$op_code,
            $crate::Modifier::$modifier,
            $reg_a,
            $crate::AddressMode::$mode_a,
            0,
            $crate::AddressMode::Direct,
        )
    };

    // Full command
    ($op_code:tt ($modifier:tt, $reg_a:expr, $mode_a:tt, $reg_b:expr, $mode_b:tt)) => {
        $crate::Instruction::new(
            $crate::OpCode::$op_code,
            $crate::Modifier::$modifier,
            $reg_a,
            $crate::AddressMode::$mode_a,
            $reg_b,
            $crate::AddressMode::$mode_b,
        )
    };
}

/// Creates a vector of instructions
/// # Example
/// ```
/// use darwin_lib::{ create_program, Instruction, AddressMode, OpCode, Modifier };
/// assert_eq!(
///    create_program! {
///        MOV(I, 0, Direct, 1, Direct)
///        JMP(None, -1, Direct)
///        DAT(None, 0, Direct, 4, Direct)
///    },
///    vec![
///        Instruction::new(OpCode::MOV, Modifier::I, 0, AddressMode::Direct, 1, AddressMode::Direct),
///        Instruction::new(OpCode::JMP, Modifier::None, -1, AddressMode::Direct, 0, AddressMode::Direct),
///        Instruction::new(OpCode::DAT, Modifier::None, 0, AddressMode::Direct, 4, AddressMode::Direct),
///    ]
/// );
/// ```
#[macro_export]
macro_rules! create_program {
    { $( $op_code:tt ($($args:tt)*) )* } => {
        vec![
            $($crate::cmd!($op_code( $($args)* )), )*
        ]
    };
}
