use std::str::Chars;

#[derive(PartialEq)]
enum Context {
    Main,
    Parameter,
    Body,
}

struct Instruction<'a> {
    pub name: &'a str,
    pub allowed_contexts: Vec<Context>,
}

struct UnrecognisedCharError {
    pub char: Option<char>,
    pub line_count: i32,
    pub position_in_line: i32,
}

struct UnrecognisedInstrError {
    pub line_count: i32,
    pub position_in_line: i32,

    pub instr_id: String,
    pub call_context: Context
}

enum SplitterErrors {
    UnrecognisedChar(UnrecognisedCharError),
    UnrecognisedInstrInContext(UnrecognisedInstrError)
}

pub struct ValueBlock {
    pub block: String,
}

pub struct InstrBlock {
    pub block: String,
    pub parameters: Vec<Block>,
}

pub struct InstrWithBodyBlock {
    pub block: String,
    pub parameters: Vec<Block>,
    pub body: Vec<Block>,
}

pub struct StructureBlock {
    pub frame_type: String,
    pub value: String,
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
    pub chars: Chars<'a>,
    pub curr_char: Option<char>,
    pub main_block: InstrWithBodyBlock,
    pub errors: Vec<SplitterErrors>,
    pub line_count: i32,
    pub position_in_line: i32,
    pub instructions: [Instruction<'a>; 2],
}

impl<'a> Splitter<'a> {
    pub fn new(content: &'a str) -> Self {
        Splitter {
            chars: content.chars(),
            curr_char: None,
            main_block: InstrWithBodyBlock {
                block: "fn".to_string(),
                parameters: vec![Block::Value(ValueBlock {
                    block: "main".to_string(),
                })],
                body: Vec::new(),
            },
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

    fn check_instr_type(&self, instr_id: String, call_context: Context) -> Option<Block> {
        if self.instructions.iter().find(|instr| -> bool {
            if instr.name == instr_id.as_str() {
                return true;
            } else {
                return false;
            }
        }).expect("{} is not currently implemented in FRAME").allowed_contexts.contains(&call_context) {
            match instr_id {
                _ => None
            }
        } else {
            self.errors.push(SplitterErrors::UnrecognisedInstrInContext(UnrecognisedInstrError {line_count: self.line_count, position_in_line: self.position_in_line, instr_id: instr_id, call_context: Context::Main }));
            None
        }
    }

    pub fn split_file(&mut self) {
        self.next_char(true);

        while self.curr_char.is_some() {
            if self.curr_char.unwrap().is_alphabetic() {
                let mut identifier = String::new();

                while self.curr_char.is_some() && self.curr_char.unwrap().is_alphanumeric() {
                    identifier.push(self.curr_char.unwrap());
                    self.next_char(false);
                }

                self.main_block.body.push(Self::check_instr_type(&self, identifier, Context::Main))
            }
            else if self.curr_char.unwrap() == '#' {
                while self.curr_char.is_some() && self.curr_char.unwrap() != '\n' {
                    self.next_char(false)
                }
            }
            else {
                self.errors
                    .push(SplitterErrors::UnrecognisedChar(UnrecognisedCharError {
                        char: self.curr_char,
                        line_count: self.line_count,
                        position_in_line: self.position_in_line,
                    }));
            }
        }
    }
}
