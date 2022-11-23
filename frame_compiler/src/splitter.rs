use std::str::Chars;

#[derive(PartialEq, Debug)]
enum Context {
    Main,
    ParameterOrStructure,
    Body,
}

impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Context::Main => write!(f, "Main"),
            Context::ParameterOrStructure => write!(f, "Parameter/Structure"),
            Context::Body => write!(f, "Body"),
        }
    }
}

struct Instruction<'a> {
    name: &'a str,
    allowed_contexts: Vec<Context>,
}

#[derive(Debug)]
struct UnknownCharError {
    char: char,
    line_count: i32,
    position_in_line: i32,
}

#[derive(Debug)]
struct UnknownInstrError {
    line_count: i32,
    position_in_line: i32,

    instr_id: String,
}

#[derive(Debug)]
struct InstrNotAllowedInContextError {
    line_count: i32,
    position_in_line: i32,

    instr_id: String,
    context: Context,
}

#[derive(Debug)]
struct UnexpectedEOFError {
    line_count: i32,
    position_in_line: i32,
}

#[derive(Debug)]
struct UnexpectedCharError {
    line_count: i32,
    position_in_line: i32,

    char: char,
    expected: String, // Information about what was expected
}

#[derive(Debug)]
enum SplitterErrors {
    UnknownChar(UnknownCharError),
    UnknownInstr(UnknownInstrError),
    InstrNotAllowedInContext(InstrNotAllowedInContextError),
    UnexpectedEOF(UnexpectedEOFError),
    UnexpectedChar(UnexpectedCharError),
}

pub struct ValueBlock {
    block: String,
}

pub struct InstrBlock {
    block: String,
    parameters: Vec<Block>,
}

pub struct InstrWithBodyBlock {
    block: String,
    parameters: Vec<Block>,
    body: Vec<Block>,
}

pub struct StructureEntry {
    var_name: String,
    frame_type: String,
    value: Option<Block>,
}

pub struct StructureBlock {
    entries: Vec<StructureEntry>,
}

pub enum Block {
    Value(ValueBlock),
    Instr(InstrBlock),
    InstrWithBody(InstrWithBodyBlock),
    Structure(StructureBlock),
}

impl Block {
    pub fn print(&self, indent: i32) {
        let indent_str = (0..indent).map(|_| " ").collect::<String>();
        match self {
            Block::Structure(block) => {
                println!("{}Block: {}", indent_str, "structure");
                for entry in block.entries.iter() {
                    println!("{}  Variable: {}", indent_str, entry.var_name);
                    println!("{}  Type: {}", indent_str, entry.frame_type);
                    if entry.value.is_some() {
                        println!("{} Value: ", indent_str);
                        entry.value.as_ref().unwrap().print(indent + 2)
                    } else {
                        println!("{} Value: Empty", indent_str);
                    }
                }
            }
            Block::Value(block) => {
                println!("{}Block: {}", indent_str, block.block)
            }
            Block::Instr(block) => {
                println!("{}Block: {}", indent_str, block.block);
                print!("{}  Params: ", indent_str);

                if block.parameters.len() == 0 {
                    println!("Empty");
                    return;
                } else {
                    println!();
                }

                for i in 0..block.parameters.len() {
                    block.parameters[i].print(indent + 2)
                }
            }
            Block::InstrWithBody(block) => {
                println!("{}Block: {}", indent_str, block.block);
                print!("{}  Params: ", indent_str);

                if block.parameters.len() == 0 {
                    println!("Empty");
                } else {
                    println!();
                }

                for i in 0..block.parameters.len() {
                    block.parameters[i].print(indent + 2)
                }

                print!("{}  Body: ", indent_str);

                if block.body.len() == 0 {
                    println!("Empty");
                } else {
                    println!();
                }

                for i in 0..block.body.len() {
                    block.body[i].print(indent + 2)
                }
            }
        };
    }
}

pub struct Splitter<'a> {
    chars: Chars<'a>,
    curr_char: char,
    errors: Vec<SplitterErrors>,
    line_count: i32,
    position_in_line: i32,
    instructions: [Instruction<'a>; 2],
}

impl<'a> Splitter<'a> {
    pub fn new(content: &'a str) -> Self {
        Splitter {
            chars: content.chars(),
            curr_char: ' ',
            errors: Vec::new(),
            line_count: 0,
            position_in_line: 0,
            instructions: [
                Instruction {
                    name: "set",
                    allowed_contexts: vec![Context::Main, Context::Body],
                },
                Instruction {
                    name: "add",
                    allowed_contexts: vec![Context::ParameterOrStructure],
                },
            ],
        }
    }

    fn next_char(&mut self, skip: bool, allow_eof: bool) -> Result<Option<char>, SplitterErrors> {
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
            return Err(SplitterErrors::UnexpectedEOF(UnexpectedEOFError {
                line_count: self.line_count,
                position_in_line: self.position_in_line,
            }));
        }

        self.curr_char = c.unwrap();
        Ok(c)
    }

    fn check_instr_type(
        &mut self,
        instr_id: String,
        call_context: Context,
    ) -> Result<Block, SplitterErrors> {
        let found_instr = self.instructions.iter().find(|instr| -> bool {
            if instr.name == instr_id.as_str() {
                return true;
            } else {
                return false;
            }
        });

        if found_instr.is_some() {
            if found_instr
                .unwrap()
                .allowed_contexts
                .contains(&call_context)
            {
                match instr_id.as_str() {
                    "set" => Ok(Block::Instr(InstrBlock {
                        block: instr_id,
                        parameters: self.split_params()?,
                    })),

                    "add" => Ok(Block::Instr(InstrBlock {
                        block: instr_id,
                        parameters: self.split_params()?,
                    })),

                    _ => panic!("lel"),
                }
            } else {
                let error = Err(SplitterErrors::InstrNotAllowedInContext(
                    InstrNotAllowedInContextError {
                        line_count: self.line_count,
                        position_in_line: self.position_in_line,
                        instr_id: instr_id,
                        context: call_context,
                    },
                ));

                error
            }
        } else {
            let error = Err(SplitterErrors::UnknownInstr(UnknownInstrError {
                line_count: self.line_count,
                position_in_line: self.position_in_line,
                instr_id: instr_id,
            }));

            error
        }
    }

    fn split_value(&mut self) -> Result<Block, SplitterErrors> {
        // Variable / instruction handling
        if self.curr_char.is_alphabetic() {
            let mut identifier = String::new();

            while self.curr_char.is_alphabetic() {
                identifier.push(self.curr_char);
                self.next_char(false, false)?;
            }

            let split = self.check_instr_type(identifier.clone(), Context::ParameterOrStructure);

            if split.is_ok() {
                self.next_char(false, false)?;
                return split;
            } else {
                return Ok(Block::Value(ValueBlock { block: identifier }));
            }
        }
        // Number handling
        else if self.curr_char.is_digit(10) || self.curr_char == '-' {
            let mut number_str = String::new();

            if self.curr_char == '-' {
                number_str.push(self.curr_char);
                self.next_char(false, false)?;
            }

            while self.curr_char.is_digit(10) || self.curr_char == '.' {
                number_str.push(self.curr_char);
                self.next_char(false, false)?;
            }

            return Ok(Block::Value(ValueBlock { block: number_str }));
        }
        // Structure handling
        else if self.curr_char == '[' {
            return Ok(Block::Structure(self.split_structure()?));
        } else {
            return Err(SplitterErrors::UnexpectedChar(UnexpectedCharError {
                line_count: self.line_count,
                position_in_line: self.position_in_line,
                char: self.curr_char,
                expected: "Expected a value".to_string(),
            }));
        }
    }

    fn split_structure(&mut self) -> Result<StructureBlock, SplitterErrors> {
        let mut structure = StructureBlock {
            entries: Vec::new(),
        };

        while self.curr_char != ']' {
            self.next_char(true, false)?;

            // Check if the first char is valid
            if !self.curr_char.is_alphabetic() {
                self.errors.push(SplitterErrors::UnexpectedChar(UnexpectedCharError {
                    line_count: self.line_count,
                    position_in_line: self.position_in_line,
                    char: self.curr_char,
                    expected: "Expected alphabetical character".to_string(),
                }));

                while !(self.curr_char == ':' || self.curr_char == ',') {
                    self.errors.push(SplitterErrors::UnexpectedChar(UnexpectedCharError {
                        line_count: self.line_count,
                        position_in_line: self.position_in_line,
                        char: self.curr_char,
                        expected: "Expected alphabetical character".to_string(),
                    }));

                    self.next_char(true, false)?;
                }

                if self.curr_char == ',' {
                    continue;
                }
            }

            // If so, extract the variable name
            let mut var_name = String::new();
            while self.curr_char.is_alphanumeric() {
                var_name.push(self.curr_char);
                self.next_char(false, false)?;
            }

            // Skip whitespace before colon
            if self.curr_char.is_whitespace() {
                self.next_char(true, false)?;
            }

            // The next character has to be a colon
            while self.curr_char != ':' {
                self.errors.push(SplitterErrors::UnexpectedChar(UnexpectedCharError {
                    line_count: self.line_count,
                    position_in_line: self.position_in_line,
                    char: self.curr_char,
                    expected: "Expected :".to_string(),
                }));

                self.next_char(true, false)?;
            }

            self.next_char(false, false)?;

            // Skip whitespace before type
            if self.curr_char.is_whitespace() {
                self.next_char(true, false)?;
            }

            // Types always start with a alphabetical character
            if !self.curr_char.is_alphabetic() {
                return Err(SplitterErrors::UnexpectedChar(UnexpectedCharError {
                    line_count: self.line_count,
                    position_in_line: self.position_in_line,
                    char: self.curr_char,
                    expected: "Expected alphabetical character".to_string(),
                }));
            }

            // Extract the type
            let mut frame_type = String::new();
            while self.curr_char.is_alphanumeric() {
                frame_type.push(self.curr_char);
                self.next_char(false, false)?;
            }

            if self.curr_char.is_whitespace() {
                self.next_char(true, false)?;
            }

            // The equals is optional
            let mut value: Option<Block> = None;
            if self.curr_char == '=' {
                self.next_char(true, false)?;

                value = Some(self.split_value()?);
            }

            // Skip whitespace before comma or closing square bracket
            if self.curr_char.is_whitespace() {
                self.next_char(true, false)?;
            }

            structure.entries.push(StructureEntry {
                var_name,
                frame_type,
                value,
            });

            if self.curr_char == ',' {
                continue;
            } else if self.curr_char == ']' {
                self.next_char(false, false)?;
                return Ok(structure);
            } else {
                while !(self.curr_char == ',' || self.curr_char == ']') {
                    self.errors.push(SplitterErrors::UnexpectedChar(UnexpectedCharError {
                        line_count: self.line_count,
                        position_in_line: self.position_in_line,
                        char: self.curr_char,
                        expected: "Expected comma or ]".to_string(),
                    }));

                    self.next_char(true, false)?;
                    }
                }
            }

        Ok(structure)
    }

    fn split_params(&mut self) -> Result<Vec<Block>, SplitterErrors> {
        let mut params: Vec<Block> = Vec::new();

        while self.curr_char != ')' {
            self.next_char(true, false)?; 

            if self.curr_char == ')' {
                return Ok(params);
            }

            let res = self.split_value();

            match res {
                Ok(res) => params.push(res),
                Err(error) => {
                    self.errors.push(error);
                    continue;
                }
            }

            if self.curr_char.is_whitespace() {
                self.next_char(true, false)?;
            }
            if self.curr_char == ',' {
                continue;
            }
            else if self.curr_char == ')' {
                return Ok(params);
            }
            else {
                self.errors
                    .push(SplitterErrors::UnknownChar(UnknownCharError {
                        char: self.curr_char,
                        line_count: self.line_count,
                        position_in_line: self.position_in_line,
                    }));
            }
        }

        Ok(params)
    }

    pub fn split_file(&mut self) -> Block {
        let mut main_block = InstrWithBodyBlock {
            block: "fn".to_string(),
            parameters: vec![Block::Value(ValueBlock {
                block: "main".to_string(),
            })],
            body: Vec::new(),
        };

        let mut c = self.next_char(true, true).unwrap_or_else(|err| {
            self.errors.push(err);
            return None;
        });

        while c.is_some() {
            if c.unwrap().is_alphabetic() {
                let mut identifier = String::new();

                while c.is_some() && c.unwrap().is_alphanumeric() {
                    identifier.push(c.unwrap());
                    c = self.next_char(false, false).unwrap_or_else(|err| {
                        self.errors.push(err);
                        return None;
                    });
                }

                let block = self.check_instr_type(identifier, Context::Main);

                if block.is_ok() {
                    main_block.body.push(block.unwrap())
                } else if block.is_err() {
                    self.errors.push(block.err().unwrap())
                }

                c = self.next_char(true, true).unwrap_or_else(|err| {
                    self.errors.push(err);
                    return None;
                });
            } else if c.unwrap() == '#' {
                while c.is_some() && c.unwrap() != '\n' {
                    c = self.next_char(false, true).unwrap_or_else(|err| {
                        self.errors.push(err);
                        return None;
                    });
                }
            } else {
                self.errors
                    .push(SplitterErrors::UnknownChar(UnknownCharError {
                        char: c.unwrap(),
                        line_count: self.line_count,
                        position_in_line: self.position_in_line,
                    }));

                c = self.next_char(true, true).unwrap_or_else(|err| {
                    self.errors.push(err);
                    return None;
                });
            }
        }

        Block::InstrWithBody(main_block)
    }

    pub fn print_errors(&self) {
        for error in self.errors.iter() {
            match error {
                SplitterErrors::UnknownChar(info) => println!(
                    "Unknown character: {} on line {}  at character {}",
                    info.char, info.line_count, info.position_in_line
                ),

                SplitterErrors::UnknownInstr(info) => println!(
                    "Unknown instruction: {} on line {} at character {}",
                    info.instr_id, info.line_count, info.position_in_line
                ),

                SplitterErrors::InstrNotAllowedInContext(info) => println!(
                    "Instruction {} is not allowed in {}: on line {} at character {}",
                    info.instr_id, info.context, info.line_count, info.position_in_line
                ),
                SplitterErrors::UnexpectedEOF(info) => println!(
                    "Unexpected End Of File on line {} at character {}",
                    info.line_count, info.position_in_line
                ),
                SplitterErrors::UnexpectedChar(info) => println!(
                    "Unexpected character {} on line {} at character {}. {}",
                    info.char, info.line_count, info.position_in_line, info.expected
                ),
            }
        }
    }
}
