use crate::{AddressMode, Modifier, OpCode, ParseError};

/// Used by the compiler to represent a parsed instruction
pub enum TokenizedLine {
    /// Used for commands that only use a single register, eg JMP.
    Single(OpCode, Modifier, isize, AddressMode),
    /// Used for commands that use both registers.
    Double(OpCode, Modifier, isize, AddressMode, isize, AddressMode),
}

pub fn tokenize_line(line: &str, line_num: usize) -> Result<TokenizedLine, ParseError> {
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.len() < 2 {
        return Err(ParseError::NotEnoughArgumets(line_num));
    }
    // getting opcode and modifier
    let first_word: Vec<&str> = words[0].split('.').collect();
    let op_code = get_opcode(first_word[0], line_num)?;
    let modifier = if first_word.len() == 2 {
        Some(get_modifier(first_word[1], line_num)?)
    } else {
        None
    };
    // Getting first value and addressing mode:
    let (mode_a, reg_a) = parse_register(words[1], line_num)?;
    // If only two words supplied, we return a TokenizedLine::Single
    if words.len() == 2 {
        // If no modifier is supplied, Singles always default to .B
        Ok(TokenizedLine::Single(
            op_code,
            modifier.unwrap_or(Modifier::B),
            reg_a,
            mode_a,
        ))
    // If three words where supplied, we parse the third word and return a TokenizedLine::Double
    } else if words.len() == 3 {
        // getting the second value and addressing mode
        let (mode_b, reg_b) = parse_register(words[2], line_num)?;
        // If no modifier is supplied, we call get_default_modifier()
        Ok(TokenizedLine::Double(
            op_code,
            modifier.unwrap_or_else(|| get_default_modifier(op_code, mode_a, mode_b)),
            reg_a,
            mode_a,
            reg_b,
            mode_b,
        ))
    } else {
        // If there are more than three words, something is wrong.
        Err(ParseError::UnexpectedArgument(line_num))
    }
}

fn parse_register(word: &str, line_num: usize) -> Result<(AddressMode, isize), ParseError> {
    match get_addressing_mode(&word[..1]) {
        Ok(v) => Ok((
            v,
            word[1..]
                .parse::<isize>()
                .map_err(|_| ParseError::UnknownValue((line_num, &word[1..])))?,
        )),
        Err(_) => Ok((
            AddressMode::Direct,
            word.parse::<isize>()
                .map_err(|_| ParseError::UnknownValue((line_num, word)))?,
        )),
    }
}

fn get_opcode(opcode: &str, line_num: usize) -> Result<OpCode, ParseError> {
    use OpCode::*;
    match opcode {
        "MOV" => Ok(MOV),
        "ADD" => Ok(ADD),
        "SUB" => Ok(SUB),
        "MUL" => Ok(MUL),
        "DIV" => Ok(DIV),
        "MOD" => Ok(MOD),
        "DAT" => Ok(DAT),
        "JMP" => Ok(JMP),
        "SPL" => Ok(SPL),
        "JMZ" => Ok(JMZ),
        "JMN" => Ok(JMN),
        "NOP" => Ok(NOP),
        "DJN" => Ok(DJN),
        "SEQ" => Ok(SEQ),
        "SNE" => Ok(SNE),
        "SLT" => Ok(SLT),
        // Anything else is an error:
        _ => Err(ParseError::UnknownOpCode((line_num, opcode))),
    }
}

fn get_modifier(modifier: &str, line_num: usize) -> Result<Modifier, ParseError> {
    use Modifier::*;
    match modifier {
        "A" => Ok(A),
        "B" => Ok(B),
        "AB" => Ok(AB),
        "BA" => Ok(BA),
        "X" => Ok(X),
        "F" => Ok(F),
        "I" => Ok(I),
        // Anything else is an error:
        _ => Err(ParseError::UnknownModifier((line_num, modifier))),
    }
}

fn get_addressing_mode(addressing_mode: &str) -> Result<AddressMode, ()> {
    use AddressMode::*;
    match addressing_mode {
        "$" => Ok(Direct),
        "#" => Ok(Immediate),
        "*" => Ok(IndirectA),
        "@" => Ok(IndirectB),
        // No addressing mode supplied:
        _ => Err(()),
    }
}

fn get_default_modifier(opcode: OpCode, mode_a: AddressMode, mode_b: AddressMode) -> Modifier {
    use OpCode as o;
    match opcode {
        o::DAT | o::NOP => Modifier::F,
        o::MOV | o::SEQ | o::SNE => {
            if mode_a == AddressMode::Immediate {
                Modifier::AB
            } else if mode_b == AddressMode::Immediate {
                Modifier::BA
            } else {
                Modifier::I
            }
        }
        o::ADD | o::SUB | o::MUL | o::DIV | o::MOD => {
            if mode_a == AddressMode::Immediate {
                Modifier::AB
            } else if mode_b == AddressMode::Immediate {
                Modifier::BA
            } else {
                Modifier::F
            }
        }
        o::SLT => {
            if mode_a == AddressMode::Immediate {
                Modifier::AB
            } else {
                Modifier::B
            }
        }
        o::JMP | o::JMZ | o::JMN | o::DJN | o::SPL => Modifier::B,
    }
}
