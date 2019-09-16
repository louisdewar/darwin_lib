use crate::{AddressMode, Instruction, Modifier, OpCode};

use std::fmt;

#[derive(Debug, PartialEq)]
enum Token {
    Opcode(OpCode),
    Modifier(Modifier),
    AddressMode(AddressMode),
    Value(isize),
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Opcode(opcode) => write!(f, "<OpCode<{:?}>>", opcode),
            Token::Modifier(modifier) => write!(f, "<Modifier<{:?}>>", modifier),
            Token::AddressMode(am) => write!(f, "<AddressMode<{}>>", am),
            Token::Value(val) => write!(f, "<Value<{}>>", val),
        }
    }
}

/// An enum for the different types of error that could occur while parsing the program.
#[derive(Debug)]
pub enum ParseError {
    /// When unexpected syntax is found. holds the line and char num where the error was found.
    Syntax((usize, usize)),
    /// When a line contains missing arguments. Holds the line number where the error was found.
    NotEnoughArgumets(usize),
    /// When a line contains an unexpected argument.
    /// Holds the line and char number where the number was found.
    UnexpectedArgument((usize, usize)),
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::Syntax(l) => write!(f, "Syntax error at {}:{}", l.0, l.1),
            ParseError::NotEnoughArgumets(l) => {
                write!(f, "Not enough arguments supplied on line {}", l)
            }
            ParseError::UnexpectedArgument(l) => {
                write!(f, "Unexpected argument found at {}:{}", l.0, l.1)
            }
        }
    }
}

/// Takes in a program as an &str, returns a vector of instructions or a ParseError.
/// # Example
/// ```
/// use darwin_lib::{Instruction, create_program, parse_program};
/// assert_eq!(
///     parse_program("MOV 0, 1").unwrap(),
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
    let (tokens, indicies) = match tokenize_line(line, line_num) {
        Ok(t) => t,
        Err(e) => return Err(e),
    };
    // A valid line must have at least 2 tokens
    if tokens.len() < 2 {
        return Err(ParseError::NotEnoughArgumets(line_num));
    }
    // Checking that the token order is valid:
    // ensures first token is the opcode:
    let opcode = match tokens[0] {
        Token::Opcode(o) => o,
        _ => return Err(ParseError::Syntax((line_num, indicies[0]))),
    };
    // JMP and SPL always have tokens: <Opcode><Value>
    if opcode == OpCode::JMP || opcode == OpCode::SPL {
        match tokens[1] {
            Token::Value(v) => {
                return Ok(Instruction::new(
                    opcode,
                    Modifier::None,
                    v,
                    AddressMode::Direct,
                    0,
                    AddressMode::Direct,
                ))
            }
            _ => return Err(ParseError::Syntax((line_num, indicies[1]))),
        }
    } else {
        // The following code is very messy. There is probarbly a better way to do this.
        if tokens.len() < 3 {
            return Err(ParseError::NotEnoughArgumets(line_num));
        }
        let mut modifier = Modifier::None;
        let mut mode_a = AddressMode::Direct;
        let mut mode_b = AddressMode::Direct;
        let value_a: isize;
        let value_b: isize;
        match tokens[1] {
            Token::Modifier(m) => {
                modifier = m;
                match tokens[2] {
                    Token::AddressMode(a) => {
                        mode_a = a;
                        match tokens[3] {
                            Token::Value(v) => {
                                value_a = v;
                                match tokens[4] {
                                    Token::AddressMode(a) => {
                                        mode_b = a;
                                        if tokens.len() < 6 {
                                            return Err(ParseError::NotEnoughArgumets(line_num));
                                        }
                                        if tokens.len() > 6 {
                                            return Err(ParseError::UnexpectedArgument((
                                                line_num,
                                                indicies[6],
                                            )));
                                        }
                                        match tokens[5] {
                                            Token::Value(v) => value_b = v,
                                            _ => {
                                                return Err(ParseError::UnexpectedArgument((
                                                    line_num,
                                                    indicies[5],
                                                )))
                                            }
                                        }
                                    }
                                    Token::Value(v) => {
                                        value_b = v;
                                        if tokens.len() > 5 {
                                            return Err(ParseError::UnexpectedArgument((
                                                line_num,
                                                indicies[5],
                                            )));
                                        }
                                    }
                                    _ => {
                                        return Err(ParseError::UnexpectedArgument((
                                            line_num,
                                            indicies[4],
                                        )))
                                    }
                                }
                            }
                            _ => {
                                return Err(ParseError::UnexpectedArgument((line_num, indicies[3])))
                            }
                        }
                    }
                    Token::Value(v) => {
                        value_a = v;
                        match tokens[3] {
                            Token::AddressMode(a) => {
                                mode_b = a;
                                if tokens.len() < 5 {
                                    return Err(ParseError::NotEnoughArgumets(line_num));
                                }
                                if tokens.len() > 5 {
                                    return Err(ParseError::UnexpectedArgument((
                                        line_num,
                                        indicies[5],
                                    )));
                                }
                                match tokens[4] {
                                    Token::Value(v) => value_b = v,
                                    _ => {
                                        return Err(ParseError::UnexpectedArgument((
                                            line_num,
                                            indicies[4],
                                        )))
                                    }
                                }
                            }
                            Token::Value(v) => {
                                value_b = v;
                                if tokens.len() > 4 {
                                    return Err(ParseError::UnexpectedArgument((
                                        line_num,
                                        indicies[4],
                                    )));
                                }
                            }
                            _ => {
                                return Err(ParseError::UnexpectedArgument((line_num, indicies[3])))
                            }
                        }
                    }
                    _ => return Err(ParseError::UnexpectedArgument((line_num, indicies[2]))),
                }
            }
            Token::AddressMode(a) => {
                mode_a = a;
                match tokens[2] {
                    Token::Value(v) => {
                        value_a = v;
                        match tokens[3] {
                            Token::AddressMode(a) => {
                                mode_b = a;
                                if tokens.len() < 5 {
                                    return Err(ParseError::NotEnoughArgumets(line_num));
                                }
                                if tokens.len() > 5 {
                                    return Err(ParseError::UnexpectedArgument((
                                        line_num,
                                        indicies[5],
                                    )));
                                }
                                match tokens[4] {
                                    Token::Value(v) => value_b = v,
                                    _ => {
                                        return Err(ParseError::UnexpectedArgument((
                                            line_num,
                                            indicies[4],
                                        )))
                                    }
                                }
                            }
                            Token::Value(v) => {
                                value_b = v;
                                if tokens.len() > 4 {
                                    return Err(ParseError::UnexpectedArgument((
                                        line_num,
                                        indicies[4],
                                    )));
                                }
                            }
                            _ => {
                                return Err(ParseError::UnexpectedArgument((line_num, indicies[3])))
                            }
                        }
                    }
                    _ => return Err(ParseError::UnexpectedArgument((line_num, indicies[2]))),
                }
            }
            Token::Value(v) => {
                value_a = v;
                match tokens[2] {
                    Token::AddressMode(a) => {
                        mode_b = a;
                        if tokens.len() < 4 {
                            return Err(ParseError::NotEnoughArgumets(line_num));
                        }
                        if tokens.len() > 4 {
                            return Err(ParseError::UnexpectedArgument((line_num, indicies[4])));
                        }
                        match tokens[3] {
                            Token::Value(v) => value_b = v,
                            _ => {
                                return Err(ParseError::UnexpectedArgument((line_num, indicies[3])))
                            }
                        }
                    }
                    Token::Value(v) => {
                        value_b = v;
                        if tokens.len() > 3 {
                            return Err(ParseError::UnexpectedArgument((line_num, indicies[3])));
                        }
                    }
                    _ => return Err(ParseError::UnexpectedArgument((line_num, indicies[2]))),
                }
            }
            _ => return Err(ParseError::UnexpectedArgument((line_num, indicies[1]))),
        }
        if modifier == Modifier::None {
            modifier = default_modifier(opcode, mode_a, mode_b);
        }
        return Ok(Instruction::new(
            opcode, modifier, value_a, mode_a, value_b, mode_b,
        ));
    }
}

fn tokenize_line(line: &str, line_num: usize) -> Result<(Vec<Token>, Vec<usize>), ParseError> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut char_iterator = line.char_indices();
    let mut word_start_index = 0;
    let mut indicies: Vec<usize> = Vec::new();
    while let Some((i, c)) = char_iterator.next() {
        match c {
            ' ' => {
                // prevents empty tokens
                if word_start_index != i {
                    let token = match tokenize_word(&line[word_start_index..i]) {
                        Ok(v) => v,
                        Err(_) => return Err(ParseError::Syntax((line_num, i))),
                    };
                    tokens.push(token);
                    indicies.push(word_start_index);
                }
                word_start_index = i + 1;
            }
            ',' | '.' => {
                let token = match tokenize_word(&line[word_start_index..i]) {
                    Ok(v) => v,
                    Err(_) => return Err(ParseError::Syntax((line_num, word_start_index))),
                };
                tokens.push(token);
                indicies.push(word_start_index);
                word_start_index = i + 1;
            }
            '$' | '#' | '*' | '@' => {
                let token = match tokenize_word(&line[word_start_index..i + 1]) {
                    Ok(v) => v,
                    Err(_) => return Err(ParseError::Syntax((line_num, word_start_index))),
                };
                tokens.push(token);
                indicies.push(word_start_index);
                word_start_index = i + 1;
            }
            _ => {}
        }
    }
    // token at end of line
    let token = match tokenize_word(&line[word_start_index..line.len()]) {
        Ok(v) => v,
        Err(_) => return Err(ParseError::Syntax((line_num, word_start_index))),
    };
    tokens.push(token);
    indicies.push(word_start_index);
    Ok((tokens, indicies))
}

fn tokenize_word(word: &str) -> Result<Token, ()> {
    // check if the word is a value:
    match word.parse::<isize>() {
        Ok(n) => return Ok(Token::Value(n)),
        Err(_) => {}
    }
    match word {
        // OpCodes:
        "MOV" => Ok(Token::Opcode(OpCode::MOV)),
        "ADD" => Ok(Token::Opcode(OpCode::ADD)),
        "SUB" => Ok(Token::Opcode(OpCode::SUB)),
        "MUL" => Ok(Token::Opcode(OpCode::MUL)),
        "DIV" => Ok(Token::Opcode(OpCode::DIV)),
        "MOD" => Ok(Token::Opcode(OpCode::MOD)),
        "DAT" => Ok(Token::Opcode(OpCode::DAT)),
        "JMP" => Ok(Token::Opcode(OpCode::JMP)),
        "SPL" => Ok(Token::Opcode(OpCode::SPL)),
        "JMZ" => Ok(Token::Opcode(OpCode::JMZ)),
        "JMN" => Ok(Token::Opcode(OpCode::JMN)),
        "NOP" => Ok(Token::Opcode(OpCode::NOP)),
        "DJN" => Ok(Token::Opcode(OpCode::DJN)),
        "SEQ" => Ok(Token::Opcode(OpCode::SEQ)),
        "SNE" => Ok(Token::Opcode(OpCode::SNE)),
        "SLT" => Ok(Token::Opcode(OpCode::SLT)),
        // Modifiers:
        "A" => Ok(Token::Modifier(Modifier::A)),
        "B" => Ok(Token::Modifier(Modifier::B)),
        "AB" => Ok(Token::Modifier(Modifier::AB)),
        "BA" => Ok(Token::Modifier(Modifier::BA)),
        "X" => Ok(Token::Modifier(Modifier::X)),
        "F" => Ok(Token::Modifier(Modifier::F)),
        "I" => Ok(Token::Modifier(Modifier::I)),
        // Addressing Modes:
        "$" => Ok(Token::AddressMode(AddressMode::Direct)),
        "#" => Ok(Token::AddressMode(AddressMode::Immediate)),
        "*" => Ok(Token::AddressMode(AddressMode::IndirectA)),
        "@" => Ok(Token::AddressMode(AddressMode::IndirectB)),
        // Anything else is an error:
        _ => Err(()),
    }
}

fn default_modifier(opcode: OpCode, mode_a: AddressMode, mode_b: AddressMode) -> Modifier {
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
