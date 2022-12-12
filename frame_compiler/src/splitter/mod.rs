use std::str::Chars;

pub mod block;
pub mod frame_type;
pub mod instr_type;
pub mod instruction;

pub struct Splitter<'a> {
    chars: Chars<'a>,
    curr_char: char,
    line_count: i32,
    position_in_line: i32,
}

impl<'a> Splitter<'a> {
    pub fn new(file: &'a String) -> Self {
        Splitter {
            chars: file.chars(),
            curr_char: ' ',
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
            return Ok(c);
        }

        if c.is_none() {
            return Err(format!(
                "Unexpected EOF character in line: {} at char {}",
                self.line_count, self.position_in_line
            ));
        }

        self.curr_char = c.unwrap();
        Ok(c)
    }
}

pub fn split_instr(splitter: &mut Splitter, identifier: String) -> Result<block::Block, String> {
    let instr_type = instr_type::toInstrType(&identifier)?;

    if let instr_type::InstrType::Regular = instr_type {
        let mut parameters: Vec<block::Block> = vec![];

        let mut c = splitter.next_char(true, false)?;

        while c.is_some() && c.unwrap() != ')' {
            parameters.push(split_parameter(splitter)?);
            c = Some(splitter.curr_char);

            if c.unwrap() == ')' {
                c = splitter.next_char(true, true)?;
                break;
            }

            c = splitter.next_char(true, false)?;
        }

        return Ok(block::Block::Instr(block::InstrBlock {
            instruction: instruction::Instructions::from_str(identifier)?,
            parameters: parameters,
        }));
    }

    Err(format!("{} is not an  instruction", identifier))
}

pub fn split_parameter(splitter: &mut Splitter) -> Result<block::Block, String> {
    if splitter.curr_char == '"' {
        let mut str = String::new();

        let mut c = splitter.next_char(false, false)?;

        while c.is_some() && c.unwrap() != '"' {
            str.push(c.unwrap());
            c = splitter.next_char(false, false)?;
        }

        c = splitter.next_char(false, false)?;

        return Ok(block::Block::PrimitiveValue(block::PrimitiveValueBlock {
            value: str,
            value_type: frame_type::FrameType::Str,
        }));
    }

    Err(format!("{} is not recognised by splitter", splitter.curr_char).to_string())
}

pub fn split_file(splitter: &mut Splitter) -> Result<Vec<block::Block>, String> {
    let mut blocks: Vec<block::Block> = vec![];

    let mut c = splitter.next_char(true, true)?;

    while c.is_some() {
        if c.unwrap().is_alphabetic() {
            let mut identifier = String::new();

            while c.is_some() && c.unwrap().is_alphanumeric() {
                identifier.push(c.unwrap());
                c = splitter.next_char(false, false)?;
            }

            blocks.push(split_instr(splitter, identifier)?);
        } else if c.unwrap() == '#' {
            while c.is_some() && c.unwrap() != '\n' {
                let mut comment = String::new();

                c = splitter.next_char(false, true)?;

                comment.push(c.unwrap());
            }
        } else {
            return Err(format!(
                "{} at position {} in line {} is not allowed",
                c.unwrap(),
                splitter.position_in_line,
                splitter.line_count
            ));
        }
    }

    Ok(blocks)
}
