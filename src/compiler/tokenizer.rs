use crate::{AddressMode, Modifier, OpCode, ParseError};

pub enum TokenizedLine {
    Single(OpCode, Modifier, isize, AddressMode),
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
        match modifier {
            Some(m) => Ok(TokenizedLine::Single(op_code, m, reg_a, mode_a)),
            None => Ok(TokenizedLine::Single(op_code, Modifier::B, reg_a, mode_a)),
        }
    // If three words where supplied, we parse the third word and return a TokenizedLine::Double
    } else if words.len() == 3 {
        // getting the second value and addressing mode
        let (mode_b, reg_b) = parse_register(words[2], line_num)?;
        // If no modifier is supplied, we call get_default_modifier()
        match modifier {
            Some(m) => Ok(TokenizedLine::Double(
                op_code, m, reg_a, mode_a, reg_b, mode_b,
            )),
            None => Ok(TokenizedLine::Double(
                op_code,
                get_default_modifier(op_code, mode_a, mode_b),
                reg_a,
                mode_a,
                reg_b,
                mode_b,
            )),
        }
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
    match opcode {
        "MOV" => Ok(OpCode::MOV),
        "ADD" => Ok(OpCode::ADD),
        "SUB" => Ok(OpCode::SUB),
        "MUL" => Ok(OpCode::MUL),
        "DIV" => Ok(OpCode::DIV),
        "MOD" => Ok(OpCode::MOD),
        "DAT" => Ok(OpCode::DAT),
        "JMP" => Ok(OpCode::JMP),
        "SPL" => Ok(OpCode::SPL),
        "JMZ" => Ok(OpCode::JMZ),
        "JMN" => Ok(OpCode::JMN),
        "NOP" => Ok(OpCode::NOP),
        "DJN" => Ok(OpCode::DJN),
        "SEQ" => Ok(OpCode::SEQ),
        "SNE" => Ok(OpCode::SNE),
        "SLT" => Ok(OpCode::SLT),
        // Anything else is an error:
        _ => Err(ParseError::UnknownOpCode((line_num, opcode))),
    }
}

fn get_modifier(modifier: &str, line_num: usize) -> Result<Modifier, ParseError> {
    match modifier {
        "A" => Ok(Modifier::A),
        "B" => Ok(Modifier::B),
        "AB" => Ok(Modifier::AB),
        "BA" => Ok(Modifier::BA),
        "X" => Ok(Modifier::X),
        "F" => Ok(Modifier::F),
        "I" => Ok(Modifier::I),
        // Anything else is an error:
        _ => Err(ParseError::UnknownModifier((line_num, modifier))),
    }
}

fn get_addressing_mode(addressing_mode: &str) -> Result<AddressMode, ()> {
    match addressing_mode {
        "$" => Ok(AddressMode::Direct),
        "#" => Ok(AddressMode::Immediate),
        "*" => Ok(AddressMode::IndirectA),
        "@" => Ok(AddressMode::IndirectB),
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
