use std::str::Chars;

#[derive(PartialEq)]
enum Context {
    Main,
    Parameter,
    Body,
}

struct Instruction<'a> {
    name: &'a str,
    allowed_contexts: Vec<Context>,
}

struct UnknownCharError {
    char: Option<char>,
    line_count: i32,
    position_in_line: i32,
}

struct UnknownInstrError {
    line_count: i32,
    position_in_line: i32,

    instr_id: String,
}

pub struct InstrNotAllowedInContextError {
    line_count: i32,
    position_in_line: i32,

    instr_id: String,
    context: Context,
}

enum SplitterErrors {
    UnknownChar(UnknownCharError),
    UnknownInstr(UnknownInstrError),
    InstrNotAllowedInContext(InstrNotAllowedInContextError),
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

pub struct StructureBlock {
    frame_type: String,
    value: String,
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
                println!("{}  Type: {}", indent_str, block.frame_type);
                println!("{}  Value: {}", indent_str, block.value);
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
    curr_char: Option<char>,
    errors: Vec<SplitterErrors>,
    line_count: i32,
    position_in_line: i32,
    instructions: [Instruction<'a>; 2],
}

impl<'a> Splitter<'a> {
    pub fn new(content: &'a str) -> Self {
        Splitter {
            chars: content.chars(),
            curr_char: None,
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
                    allowed_contexts: vec![Context::Parameter],
                },
            ],
        }
    }

    fn next_char(&mut self, skip: bool) {
        self.curr_char = self.chars.next();
        self.position_in_line += 1;

        while skip && self.curr_char.is_some() && self.curr_char.unwrap().is_whitespace() {
            if self.curr_char.unwrap() == '\n' {
                self.position_in_line = 0;
                self.line_count += 1;
            }
            self.curr_char = self.chars.next();
            self.position_in_line += 1;
        }
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

                    _ => panic!("yes"),
                }
            } else {
                let _ = self.split_params();

                Err(SplitterErrors::InstrNotAllowedInContext(
                    InstrNotAllowedInContextError {
                        line_count: self.line_count,
                        position_in_line: self.position_in_line,
                        instr_id: instr_id,
                        context: call_context,
                    },
                ))
            }
        } else {
            let _ = self.split_params();

            Err(SplitterErrors::UnknownInstr(UnknownInstrError {
                line_count: self.line_count,
                position_in_line: self.position_in_line,
                instr_id: instr_id,
            }))
        }
    }

    fn split_params(&mut self) -> Result<Vec<Block>, SplitterErrors> {
        let mut params: Vec<Block> = Vec::new();

        self.next_char(true);

        while self.curr_char.is_some() && self.curr_char.unwrap() != ')' {
            if self.curr_char.unwrap().is_digit(10) || self.curr_char.unwrap() == '-' {
                let mut number_str = String::new();

                while self.curr_char.is_some() && self.curr_char.unwrap().is_digit(10)
                    || self.curr_char.unwrap() == '.'
                {
                    number_str.push(self.curr_char.unwrap());
                    self.next_char(false);
                }

                params.push(Block::Value(ValueBlock { block: number_str }));
            }

            if self.curr_char.unwrap().is_whitespace() || self.curr_char.unwrap() == ',' {
                self.next_char(true);
            } else if self.curr_char.unwrap() == ')' {
                return Ok(params);
            } else {
                return Err(SplitterErrors::UnknownChar(UnknownCharError {
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

        self.next_char(true);

        while self.curr_char.is_some() {
            if self.curr_char.unwrap().is_alphabetic() {
                let mut identifier = String::new();

                while self.curr_char.is_some() && self.curr_char.unwrap().is_alphanumeric() {
                    identifier.push(self.curr_char.unwrap());
                    self.next_char(false);
                }

                let block = self.check_instr_type(identifier, Context::Main);

                match block {
                    Ok(block) => main_block.body.push(block),

                    Err(error) => self.errors.push(error),
                }
            } else if self.curr_char.unwrap() == '#' {
                while self.curr_char.is_some() && self.curr_char.unwrap() != '\n' {
                    self.next_char(false)
                }
            } else {
                self.errors
                    .push(SplitterErrors::UnknownChar(UnknownCharError {
                        char: self.curr_char,
                        line_count: self.line_count,
                        position_in_line: self.position_in_line,
                    }));
            }

            self.next_char(true);
        }

        Block::InstrWithBody(main_block)
    }
}
