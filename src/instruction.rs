#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AddressMode {
    Direct,
    Immediate,
    // TODO: Add more modes
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    MOV(isize, AddressMode, isize, AddressMode),
    DAT(isize, AddressMode, isize, AddressMode),
    JMP(isize, AddressMode),
}

/// Creates a single instruction
/// # Example
/// ```
/// use darwin_lib::{ cmd, Instruction, AddressMode };
/// // MOV instruction
/// assert_eq!(cmd! { MOV(0, Direct, 1, Direct) }, Instruction::MOV(0, AddressMode::Direct, 1, AddressMode::Direct));
///
/// // DAT instruction
/// assert_eq!(cmd! { DAT(0, Direct, 1, Direct) }, Instruction::DAT(0, AddressMode::Direct, 1, AddressMode::Direct));
///
/// // JMP instruction
/// assert_eq!(cmd! { JMP(0, Direct) }, Instruction::JMP(0, AddressMode::Direct));
/// ```
#[macro_export]
macro_rules! cmd {
    // Single parameter
    ($op_code:tt ($reg_a:expr, $mode_a:tt)) => {
        $crate::Instruction::$op_code($reg_a, $crate::AddressMode::$mode_a)
    };

    // Full command
    ($op_code:tt ($reg_a:expr, $mode_a:tt, $reg_b:expr, $mode_b:tt)) => {
        $crate::Instruction::$op_code(
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
/// use darwin_lib::{ create_program, Instruction, AddressMode };
/// assert_eq!(
///    create_program! {
///        MOV(0, Direct, 1, Direct)
///        JMP(-1, Direct)
///        DAT(0, Direct, 4, Direct)
///    },
///    vec![
///        Instruction::MOV(0, AddressMode::Direct, 1, AddressMode::Direct),
///        Instruction::JMP(-1, AddressMode::Direct),
///        Instruction::DAT(0, AddressMode::Direct, 4, AddressMode::Direct),
///    ]
/// );
/// ```
#[macro_export]
macro_rules! create_program {
    // { $( $op_code:tt $reg_a:tt $mode_a:tt $reg_b:tt $mode_b:tt ),* } => {
    { $( $op_code:tt ($($args:tt)*) )* } => {
        // vec![$( $crate::cmd!($op_code ($($args)*))),*]
        vec![
            $($crate::cmd!($op_code( $($args)* )), )*
        ]
    };
}
