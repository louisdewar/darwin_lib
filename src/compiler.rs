use crate::{AddressMode, Instruction, Modifier, OpCode};

mod tokenizer;
use tokenizer::{tokenize_line, TokenizedLine};

use std::fmt;

/// An enum for the different types of error that could occur while compiling a program.
#[derive(Debug)]
pub enum ParseError<'a> {
    /// When an instruction uses a modifier that is not compatable with the opcode.
    /// Holds the line num where the error was found.
    InvalidModifier(usize),
    /// When a line contains missing arguments. Holds the line number where the error was found.
    NotEnoughArgumets(usize),
    /// When a line contains an unexpected argument.
    /// Holds the line number where the error was found.
    UnexpectedArgument(usize),
    /// When a supplied OpCode is not recognised.
    /// Holds the line number where the error was found and the string that caused the error.
    UnknownOpCode((usize, &'a str)),
    /// When a supplied Modifier is not recognised.
    /// Holds the line number where the error was found and the string that caused the error.
    UnknownModifier((usize, &'a str)),
    /// When a supplied value cannot be parsed into an isize.
    /// Holds the line number where the error was found and the value that caused the error.
    UnknownValue((usize, &'a str)),
}
impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidModifier(l) => write!(f, "An incompatable modifier / OpCode combo was used on line {}", l),
            ParseError::NotEnoughArgumets(l) => {
                write!(f, "Not enough arguments supplied on line {}", l)
            }
            ParseError::UnexpectedArgument(l) => {
                write!(f, "Unexpected argument found on line {}", l)
            }
            ParseError::UnknownOpCode(l) => {
                write!(f, "Unknown OpCode '{}' found on line {}", l.1, l.0)
            }
            ParseError::UnknownModifier(l) => {
                write!(f, "Unknown modifier '{}' found on line {}", l.1, l.0)
            }
            ParseError::UnknownValue(l) => {
                write!(f, "The value '{}' found on line {} is not valid", l.1, l.0)
            }
        }
    }
}

/// Takes in a program as an &str, returns a vector of instructions or a ParseError.
/// # Example
/// ```
/// use darwin_lib::{Instruction, create_program, parse_program};
/// assert_eq!(
///     parse_program("MOV.I 0 1").unwrap(),
///     create_program!(MOV(I, 0, Direct, 1, Direct))
/// )
/// ```
pub fn parse_program(program: &str) -> Result<Vec<Instruction>, ParseError> {
    let mut parsed_program: Vec<Instruction> = Vec::new();
    for (i, line) in program.lines().enumerate() {
        match parse_line(&line, i + 1) {
            Ok(v) => {
                parsed_program.push(v);
            }
            Err(e) => return Err(e),
        }
    }
    Ok(parsed_program)
}

fn parse_line(line: &str, line_num: usize) -> Result<Instruction, ParseError> {
    let tokenized_line = match tokenize_line(line, line_num) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };
    let (op_code, modifier, reg_a, mode_a, reg_b, mode_b) = match tokenized_line {
        TokenizedLine::Single(op_code, modifier, reg_a, mode_a) => {
            // Check to see if valid op_code for a single parameter
            if match op_code {
                OpCode::JMP => true,
                OpCode::SPL => true,
                _ => false
            } {
                (op_code, modifier, reg_a, mode_a, 0, AddressMode::Direct)
            } else {
                // Needed 2 params got 1
                return Err(ParseError::NotEnoughArgumets(line_num))
            }
        }
        TokenizedLine::Double(op_code, modifier, reg_a, mode_a, reg_b, mode_b) => {
            (op_code, modifier, reg_a, mode_a, reg_b, mode_b)
        }
    };
    if modifier == Modifier::I && op_code != OpCode::MOV {
        return Err(ParseError::InvalidModifier(line_num))
    }
    Ok(Instruction::new(op_code, modifier, reg_a, mode_a, reg_b, mode_b))
}
