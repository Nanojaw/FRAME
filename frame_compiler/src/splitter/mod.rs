use std::{ops::Deref, str::Chars};

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
    let instr_type = instr_type::to_instr_type(&identifier)?;

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
    } else {
        if identifier == "fn" {
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

            let mut fn_name = String::new();
            if let block::Block::PrimitiveValue(pV) = &parameters[0] {
                if let frame_type::FrameType::Variable = pV.value_type {
                    fn_name = pV.value.to_string();
                }
            }

            let mut fn_return_type: frame_type::FrameType = frame_type::FrameType::Bool;
            if let block::Block::PrimitiveValue(pV) = &parameters[parameters.len() - 1] {
                if let frame_type::FrameType::Variable = pV.value_type {
                    let fn_return_type_test = frame_type::FrameType::which(pV.value.to_string());

                    fn_return_type = match fn_return_type_test {
                        Ok(fn_return_type) => fn_return_type,
                        Err(_) => {
                            let array_end_type = frame_type::FrameType::which(
                                pV.value.split('_').collect::<Vec<&str>>()[1].to_string(),
                            )?;

                            let dims = &pV.value[5..5 + (pV.value.find('d').unwrap() - 5)]
                                .parse::<i8>()
                                .unwrap();

                            frame_type::FrameType::Array(frame_type::ArrayType {
                                dimensions: *dims,
                                values_type: Box::new(array_end_type),
                            })
                        }
                    };
                }
            }

            let mut fn_params: Vec<block::StructureBlock> = vec![];
            for i in 1..parameters.len() - 1 {
                if let block::Block::Structure(structure) = &parameters[i] {
                    fn_params.push(structure.clone());
                }
            }

            splitter.next_char(true, false)?;

            let mut fn_body: Vec<block::Block> = vec![];

            while splitter.curr_char.is_some() && splitter.curr_char.unwrap() != '}' {
                if splitter.curr_char.unwrap().is_alphabetic() {
                    let mut identifier = String::new();

                    while splitter.curr_char.is_some()
                        && splitter.curr_char.unwrap().is_alphanumeric()
                    {
                        identifier.push(splitter.curr_char.unwrap());
                        splitter.next_char(false, false)?;
                    }

                    fn_body.push(split_instr(splitter, identifier)?);
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

                    fn_body.push(block::Block::Comment(block::CommentBlock {
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

            return Ok(block::Block::Function(block::FunctionBlock {
                name: fn_name,
                parameters: fn_params,
                body: fn_body,
                return_type: fn_return_type,
            }));
        } else {
            let instr = instruction::InstructionsWithBody::from_str(identifier.to_string())?;

            splitter.next_char(true, false)?;

            let mut params: Vec<block::Block> = vec![];

            if splitter.curr_char.is_some() && splitter.curr_char.unwrap() != ')' {
                while splitter.curr_char.is_some() && splitter.curr_char.unwrap() != ')' {
                    params.push(split_parameter(splitter)?);
    
                    if splitter.curr_char.unwrap() == ')' {
                        break;
                    }
                }
            }

            splitter.next_char(true, false)?;
            splitter.next_char(true, false)?;

            let mut instr_body: Vec<block::Block> = vec![];

            while splitter.curr_char.is_some() && splitter.curr_char.unwrap() != '}' {
                if splitter.curr_char.unwrap().is_alphabetic() {
                    let mut identifier = String::new();

                    while splitter.curr_char.is_some()
                        && splitter.curr_char.unwrap().is_alphanumeric()
                    {
                        identifier.push(splitter.curr_char.unwrap());
                        splitter.next_char(false, false)?;
                    }

                    instr_body.push(split_instr(splitter, identifier)?);
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

                    instr_body.push(block::Block::Comment(block::CommentBlock {
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
            splitter.next_char(true, false)?;
            return Ok(block::Block::InstrWithBody(block::InstrWithBodyBlock { instruction: instr, parameters: params, body: instr_body }));
        }
    }
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
        } else if splitter.curr_char.unwrap() == '[' {
            splitter.next_char(true, false)?;
            let var_name_block = split_parameter(splitter)?;

            let mut var_name = String::new();
            if let block::Block::PrimitiveValue(pv) = var_name_block {
                var_name = pv.value;
            }

            splitter.next_char(true, false)?;

            let mut type_str = String::new();
            while splitter.curr_char.is_some() && splitter.curr_char.unwrap().is_alphabetic()
                || splitter.curr_char.unwrap().is_digit(10)
                || splitter.curr_char.unwrap() == '_'
            {
                type_str.push(splitter.curr_char.unwrap());
                splitter.next_char(false, false)?;
            }

            let mut var_type: frame_type::FrameType;

            let var_type_test = frame_type::FrameType::which(type_str.clone());
            if var_type_test.is_err() {
                let array_end_type = frame_type::FrameType::which(
                    type_str.split('_').collect::<Vec<&str>>()[1].to_string(),
                )?;

                let dims = &type_str[5..5 + (type_str.find('d').unwrap() - 5)]
                    .parse::<i8>()
                    .unwrap();

                var_type = frame_type::FrameType::Array(frame_type::ArrayType {
                    dimensions: *dims,
                    values_type: Box::new(array_end_type),
                })
            } else {
                var_type = var_type_test.unwrap();
            }

            if splitter.curr_char.is_some() && splitter.curr_char.unwrap() == ']' {
                splitter.next_char(true, false)?;

                return Ok(block::Block::Structure(block::StructureBlock {
                    var_name: var_name,
                    var_type: var_type,
                    var_value: Box::new(None),
                }));
            }

            splitter.next_char(true, false)?;

            if splitter.curr_char.is_some() && splitter.curr_char.unwrap() == ']' {
                splitter.next_char(true, false)?;

                return Ok(block::Block::Structure(block::StructureBlock {
                    var_name: var_name,
                    var_type: var_type,
                    var_value: Box::new(None),
                }));
            }

            if splitter.curr_char.is_some() && splitter.curr_char.unwrap() == '=' {
                splitter.next_char(true, false)?;

                let var_value = split_parameter(splitter)?;

                splitter.next_char(true, false)?;

                return Ok(block::Block::Structure(block::StructureBlock {
                    var_name: var_name,
                    var_type: var_type,
                    var_value: Box::new(Some(var_value)),
                }));
            }
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
            splitter.next_char(true, true)?;
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
