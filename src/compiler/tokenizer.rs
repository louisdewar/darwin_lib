use crate::{AddressMode, Modifier, OpCode, ParseError};

pub enum TokenizedLine {
    Single(OpCode, Modifier, isize, AddressMode),
    Double(OpCode, Modifier, isize, AddressMode, isize, AddressMode),
}

pub fn tokenize_line(line: &str, line_num: usize) -> Result<TokenizedLine, ParseError> {
    //let mut tokens: Vec<Token> = Vec::new();
    let words: Vec<&str> = line.split_whitespace().collect();
    if words.len() < 2 {
        return Err(ParseError::NotEnoughArgumets(line_num));
    }
    // getting opcode and modifier
    let first_word: Vec<&str> = words[0].split('.').collect();
    let op_code = match get_opcode(first_word[0]) {
        Ok(v) => v,
        Err(_) => return Err(ParseError::UnknownOpCode((line_num, &first_word[0]))),
    };
    let modifier = if first_word.len() == 2 {
        match get_modifier(first_word[1]) {
            Ok(v) => Some(v),
            Err(_) => return Err(ParseError::UnknownModifier((line_num, &first_word[1]))),
        }
    } else {
        None
    };
    // getting first value and addressing mode
    let value_1;
    let addressing_mode_1;
    match get_addressing_mode(&words[1][..1]) {
        Ok(v) => {
            addressing_mode_1 = v;
            value_1 = match words[1][1..].parse::<isize>() {
                Ok(n) => n,
                Err(_) => return Err(ParseError::UnknownValue((line_num, &words[1][1..]))),
            };
        }
        Err(_) => {
            addressing_mode_1 = AddressMode::Direct;
            value_1 = match words[1].parse::<isize>() {
                Ok(n) => n,
                Err(_) => return Err(ParseError::UnknownValue((line_num, &words[1]))),
            };
        }
    }
    if words.len() == 2 {
        match modifier {
            Some(m) => Ok(TokenizedLine::Single(
                op_code,
                m,
                value_1,
                addressing_mode_1,
            )),
            None => Ok(TokenizedLine::Single(
                op_code,
                Modifier::B,
                value_1,
                addressing_mode_1,
            )),
        }
    } else if words.len() == 3 {
        // getting the second value and addressing mode
        let value_2;
        let addressing_mode_2;
        match get_addressing_mode(&words[2][..1]) {
            Ok(v) => {
                addressing_mode_2 = v;
                value_2 = match words[2][1..].parse::<isize>() {
                    Ok(n) => n,
                    Err(_) => return Err(ParseError::UnknownValue((line_num, &words[2][1..]))),
                };
            }
            Err(_) => {
                addressing_mode_2 = AddressMode::Direct;
                value_2 = match words[2].parse::<isize>() {
                    Ok(n) => n,
                    Err(_) => return Err(ParseError::UnknownValue((line_num, &words[2]))),
                };
            }
        }
        match modifier {
            Some(m) => Ok(TokenizedLine::Double(
                op_code,
                m,
                value_1,
                addressing_mode_1,
                value_2,
                addressing_mode_2,
            )),
            None => Ok(TokenizedLine::Double(
                op_code,
                get_default_modifier(op_code, addressing_mode_1, addressing_mode_2),
                value_1,
                addressing_mode_1,
                value_2,
                addressing_mode_2,
            )),
        }
    } else {
        Err(ParseError::UnexpectedArgument(line_num))
    }
}

fn get_opcode(opcode: &str) -> Result<OpCode, ()> {
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
        _ => Err(()),
    }
}

fn get_modifier(modifier: &str) -> Result<Modifier, ()> {
    match modifier {
        "A" => Ok(Modifier::A),
        "B" => Ok(Modifier::B),
        "AB" => Ok(Modifier::AB),
        "BA" => Ok(Modifier::BA),
        "X" => Ok(Modifier::X),
        "F" => Ok(Modifier::F),
        "I" => Ok(Modifier::I),
        // Anything else is an error:
        _ => Err(()),
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
