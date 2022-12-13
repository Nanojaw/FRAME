use std::str::Chars;

pub mod block;
pub mod frame_type;
pub mod instr_type;
pub mod instruction;

pub struct Splitter<'a> {
    chars: Chars<'a>,
    curr_char: Option<char>,
    line_count: i32,
    position_in_line: i32,
}

impl<'a> Splitter<'a> {
    pub fn new(file: &'a str) -> Self {
        Splitter {
            chars: file.chars(),
            curr_char: None,
            line_count: 1,
            position_in_line: 0,
        }
    }

    pub fn next_char(&mut self, skip: bool, allow_eof: bool) -> Result<Option<char>, String> {
        let mut c = self.chars.next();
        self.position_in_line += 1;

        while skip && c.is_some() && c.unwrap().is_whitespace() {
            if c.unwrap() == '\n' {
                self.position_in_line = 0;
                self.line_count += 1;
            }
            c = self.chars.next();
            self.position_in_line += 1;
        }

        if allow_eof {
            self.curr_char = c;
            return Ok(c);
        }

        if c.is_none() {
            return Err(format!(
                "Unexpected EOF character in line: {} at char {}",
                self.line_count, self.position_in_line
            ));
        }

        self.curr_char = Some(c.unwrap());
        Ok(c)
    }
}

pub fn split_instr(splitter: &mut Splitter, identifier: String) -> Result<block::Block, String> {
    let instr_type = instr_type::toInstrType(&identifier)?;

    if let instr_type::InstrType::Regular = instr_type {
        let mut parameters: Vec<block::Block> = vec![];

        splitter.next_char(true, false)?;

        while splitter.curr_char.is_some() && splitter.curr_char.unwrap() != ')' {
            parameters.push(split_parameter(splitter)?);

            if splitter.curr_char.unwrap() == ')' {
                splitter.next_char(true, true)?;
                break;
            }

            splitter.next_char(true, false)?;
        }

        return Ok(block::Block::Instr(block::InstrBlock {
            instruction: instruction::Instructions::from_str(identifier)?,
            parameters: parameters,
        }));
    }

    Err(format!("{} is not an  instruction", identifier))
}

pub fn split_parameter(splitter: &mut Splitter) -> Result<block::Block, String> {
    if splitter.curr_char.is_some() {
        if splitter.curr_char.unwrap().is_alphabetic() {
            let mut identifier = String::new();

            while splitter.curr_char.is_some() && splitter.curr_char.unwrap().is_alphanumeric() {
                identifier.push(splitter.curr_char.unwrap());
                splitter.next_char(false, false)?;
            }

            if identifier == "true" || identifier == "false" {
                if identifier == "true" {
                    return Ok(block::Block::PrimitiveValue(block::PrimitiveValueBlock {
                        value: identifier,
                        value_type: frame_type::FrameType::Bool,
                    }));
                } else if identifier == "false" {
                    return Ok(block::Block::PrimitiveValue(block::PrimitiveValueBlock {
                        value: identifier,
                        value_type: frame_type::FrameType::Bool,
                    }));
                }
            }

            let test = split_instr(splitter, identifier.clone());

            return match test {
                Ok(block) => Ok(block),
                Err(_) => Ok(block::Block::PrimitiveValue(block::PrimitiveValueBlock {
                    value: identifier,
                    value_type: frame_type::FrameType::Variable,
                })),
            };
        } else if splitter.curr_char.unwrap() == '"' {
            let mut str = String::new();

            splitter.next_char(false, false)?;

            while splitter.curr_char.is_some() && splitter.curr_char.unwrap() != '"' {
                str.push(splitter.curr_char.unwrap());
                splitter.next_char(false, false)?;
            }

            splitter.next_char(false, false)?;

            return Ok(block::Block::PrimitiveValue(block::PrimitiveValueBlock {
                value: str,
                value_type: frame_type::FrameType::Str,
            }));
        } else if splitter.curr_char.unwrap().is_digit(10) || splitter.curr_char.unwrap() == '-' {
            let mut num_str = String::new();

            while splitter.curr_char.is_some() && splitter.curr_char.unwrap().is_digit(10)
                || splitter.curr_char.unwrap() == '-'
                || splitter.curr_char.unwrap() == '.'
            {
                num_str.push(splitter.curr_char.unwrap());
                splitter.next_char(false, false)?;
            }

            if num_str.contains('-') {
                return Ok(block::Block::PrimitiveValue(block::PrimitiveValueBlock {
                    value: num_str,
                    value_type: frame_type::FrameType::Signed,
                }));
            } else if num_str.contains('.') {
                return Ok(block::Block::PrimitiveValue(block::PrimitiveValueBlock {
                    value: num_str,
                    value_type: frame_type::FrameType::Float,
                }));
            } else {
                return Ok(block::Block::PrimitiveValue(block::PrimitiveValueBlock {
                    value: num_str,
                    value_type: frame_type::FrameType::Unsigned,
                }));
            }
        } else if splitter.curr_char.unwrap() == '{' {
            splitter.next_char(true, false)?;

            let mut contents: Vec<block::Block> = vec![];

            while splitter.curr_char.is_some() && splitter.curr_char.unwrap() != '}' {
                contents.push(split_parameter(splitter)?);

                if splitter.curr_char.is_some() && splitter.curr_char.unwrap() == '}' {
                    splitter.next_char(true, true)?;

                    return Ok(block::Block::Array(block::ArrayBlock { values: contents }));
                }

                splitter.next_char(true, false)?;
            }

            splitter.next_char(true, true)?;

            return Ok(block::Block::Array(block::ArrayBlock { values: contents }));
        }
    }

    Err(format!(
        "{} is not recognised by splitter",
        splitter.curr_char.unwrap()
    )
    .to_string())
}

pub fn split_file(splitter: &mut Splitter) -> Result<Vec<block::Block>, String> {
    let mut blocks: Vec<block::Block> = vec![];

    splitter.next_char(true, true)?;

    while splitter.curr_char.is_some() {
        if splitter.curr_char.unwrap().is_alphabetic() {
            let mut identifier = String::new();

            while splitter.curr_char.is_some() && splitter.curr_char.unwrap().is_alphanumeric() {
                identifier.push(splitter.curr_char.unwrap());
                splitter.next_char(false, false)?;
            }

            blocks.push(split_instr(splitter, identifier)?);
        } else if splitter.curr_char.unwrap() == '#' {
            let mut comment = String::new();

            while splitter.curr_char.is_some() && splitter.curr_char.unwrap() != '\n' {
                splitter.next_char(false, true)?;

                if splitter.curr_char.is_some() && splitter.curr_char.unwrap() == '\n' {
                    splitter.next_char(true, true)?;
                    break;
                }

                comment.push(splitter.curr_char.unwrap());
            }

            blocks.push(block::Block::Comment(block::CommentBlock {
                value: comment,
            }));
        } else {
            return Err(format!(
                "{} at position {} in line {} is not allowed",
                splitter.curr_char.unwrap(),
                splitter.position_in_line,
                splitter.line_count
            ));
        }
    }

    Ok(blocks)
}
