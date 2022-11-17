use std::str::Chars;

pub enum CalledFrom {
    WithinMain,
    WithinParam,
    WithinBody,
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

trait StringMagicForFrame {
    fn is_string<'a>(&'a self) -> &'a bool;
    fn is_frame_digit<'a>(&'a self) -> &'a bool;
}

impl StringMagicForFrame for String {
    fn is_string<'a>(&'a self) -> &'a bool {
        let mut chars = self.chars();

        let mut c = chars.next();

        while c.is_some() {
            if !c.unwrap().is_alphabetic() {
                return &false;
            }

            c = chars.next();
        }
        &true
    }

    fn is_frame_digit<'a>(&'a self) -> &'a bool {
        let mut chars = self.chars();

        let mut c = chars.next();

        while c.is_some() {
            if !c.unwrap().is_digit(10) && c.unwrap() != '.' {
                return &false;
            }

            c = chars.next();
        }

        &true
    }
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

fn check_input_type(identifier: &str, location: CalledFrom, chars: &mut Chars) -> Option<Block> {
    match location {
        CalledFrom::WithinMain => match identifier {
            // Regular instructions
            "set" => Some(Block::Instr(InstrBlock {
                block: "set".to_string(),
                parameters: split_params(chars),
            })),

            "do" => Some(Block::Instr(InstrBlock {
                block: "do".to_string(),
                parameters: split_params(chars),
            })),

            // Flow control
            "if" => Some(Block::InstrWithBody(InstrWithBodyBlock {
                block: "if".to_string(),
                parameters: split_params(chars),
                body: split_body(chars),
            })),

            "else" => Some(Block::InstrWithBody(InstrWithBodyBlock {
                block: "else".to_string(),
                parameters: vec![],
                body: split_body(chars),
            })),

            "for" => Some(Block::InstrWithBody(InstrWithBodyBlock {
                block: "for".to_string(),
                parameters: split_params(chars),
                body: split_body(chars),
            })),

            "while" => Some(Block::InstrWithBody(InstrWithBodyBlock {
                block: "while".to_string(),
                parameters: split_params(chars),
                body: split_body(chars),
            })),

            // Idk lol
            "mod" => Some(Block::InstrWithBody(InstrWithBodyBlock {
                block: "mod".to_string(),
                parameters: split_params(chars),
                body: split_body(chars),
            })),

            // Function stuff
            "fn" => Some(Block::InstrWithBody(InstrWithBodyBlock {
                block: "fn".to_string(),
                parameters: split_params(chars),
                body: split_body(chars),
            })),

            "return" => Some(Block::Instr(InstrBlock {
                block: "return".to_string(),
                parameters: split_params(chars),
            })),

            // Stuff that is not implemented or error ridden
            _ => {
                println!("Unknown instruction in main \"{identifier}\"");
                return None;
            }
        },
        CalledFrom::WithinParam => match identifier {
            "do" => Some(Block::Instr(InstrBlock {
                block: "do".to_string(),
                parameters: split_params(chars),
            })),
            
            // Math stuff
            "add" => Some(Block::Instr(InstrBlock {
                block: "add".to_string(),
                parameters: split_params(chars),
            })),

            "sub" => Some(Block::Instr(InstrBlock {
                block: "sub".to_string(),
                parameters: split_params(chars),
            })),

            "mul" => Some(Block::Instr(InstrBlock {
                block: "mul".to_string(),
                parameters: split_params(chars),
            })),

            "div" => Some(Block::Instr(InstrBlock {
                block: "div".to_string(),
                parameters: split_params(chars),
            })),

            // Conditionals
            "not" => Some(Block::Instr(InstrBlock {
                block: "not".to_string(),
                parameters: split_params(chars),
            })),

            "eq" => Some(Block::Instr(InstrBlock {
                block: "eq".to_string(),
                parameters: split_params(chars),
            })),

            "neq" => Some(Block::Instr(InstrBlock {
                block: "neq".to_string(),
                parameters: split_params(chars),
            })),

            "greater" => Some(Block::Instr(InstrBlock {
                block: "greater".to_string(),
                parameters: split_params(chars),
            })),

            "less" => Some(Block::Instr(InstrBlock {
                block: "less".to_string(),
                parameters: split_params(chars),
            })),

            "and" => Some(Block::Instr(InstrBlock {
                block: "and".to_string(),
                parameters: split_params(chars),
            })),

            "or" => Some(Block::Instr(InstrBlock {
                block: "eq".to_string(),
                parameters: split_params(chars),
            })),

            // Variable or litteral
            _ => Some(Block::Value(ValueBlock {
                block: identifier.to_string(),
            })),
        },
        CalledFrom::WithinBody => match identifier {
            // Normal instructions
            "set" => Some(Block::Instr(InstrBlock {
                block: "set".to_string(),
                parameters: split_params(chars),
            })),

            "do" => Some(Block::Instr(InstrBlock {
                block: "do".to_string(),
                parameters: split_params(chars),
            })),

            // Flow control
            "if" => Some(Block::InstrWithBody(InstrWithBodyBlock {
                block: "if".to_string(),
                parameters: split_params(chars),
                body: split_body(chars),
            })),

            "else" => Some(Block::InstrWithBody(InstrWithBodyBlock {
                block: "else".to_string(),
                parameters: vec![],
                body: split_body(chars),
            })),

            "for" => Some(Block::InstrWithBody(InstrWithBodyBlock {
                block: "for".to_string(),
                parameters: split_params(chars),
                body: split_body(chars),
            })),

            "while" => Some(Block::InstrWithBody(InstrWithBodyBlock {
                block: "while".to_string(),
                parameters: split_params(chars),
                body: split_body(chars),
            })),

            "return" => Some(Block::Instr(InstrBlock {
                block: "return".to_string(),
                parameters: split_params(chars),
            })),

            // error stuff
            _ => {
                println!("Unknown instruction in body \"{identifier}\"");
                return None;
            }
        },
    }
}

fn next_char(chars: &mut Chars) -> Option<char> {
    let mut c = chars.next();

    while c.is_some() && c.unwrap().is_whitespace() {
        c = chars.next();
    }

    if c.is_some() {
        return c;
    }

    None
}

fn split_structure(chars: &mut Chars) -> Option<Block> {
    let mut c = chars.next();

    let mut type_str = String::new();

    while c.is_some() && c.unwrap() != ',' {
        type_str.push(c.unwrap());
        c = chars.next();
    }

    c = next_char(chars);

    let mut value_str = String::new();

    while c.is_some() && c.unwrap() != ']' {
        value_str.push(c.unwrap());
        c = chars.next();
    }

    if value_str.is_string().to_owned() || value_str.is_frame_digit().to_owned() {
        return Some(Block::Structure(StructureBlock {
            frame_type: type_str,
            value: value_str,
        }));
    }

    None
}

fn split_params(chars: &mut Chars) -> Vec<Block> {
    let mut c = chars.next();
    let mut params: Vec<Block> = Vec::new();

    while c.is_some() && c.unwrap() != ')' {
        if c.unwrap().is_digit(10) || c.unwrap() == '-' {
            let mut number_str = String::new();

            while c.is_some() && (c.unwrap().is_digit(10) || c.unwrap() == '.') || c.unwrap() == '-'
            {
                number_str.push(c.unwrap());
                c = chars.next();
            }

            params.push(Block::Value(ValueBlock { block: number_str }));
        }

        if c.unwrap().is_alphabetic() {
            let mut identifier = String::new();

            while c.is_some() && c.unwrap().is_alphanumeric() {
                identifier.push(c.unwrap());
                c = chars.next();
            }

            params.push(
                check_input_type(identifier.as_str(), CalledFrom::WithinParam, chars).unwrap(),
            );

            if c.unwrap() != ',' && c.unwrap() != ')' {
                c = next_char(chars)
            }
        }

        if c.unwrap() == '[' {
            let split_structure = split_structure(chars);

            if split_structure.is_some() {
                params.push(split_structure.unwrap());
            } else {
                println!("Could not split structure");
            }

            c = chars.next();
        }

        if c.is_some() && c.unwrap() == ',' {
            c = next_char(chars);
            continue;
        } else if c.is_some() && c.unwrap() == ')' {
            return params;
        }
    }

    params
}

fn split_body(chars: &mut Chars) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();

    let mut c = next_char(chars);

    if c.is_some() && c.unwrap() == '{' {
        c = next_char(chars)
    } else {
        // error handling
    }

    while c.is_some() && c.unwrap() != '}' {
        let mut identifier = String::new();

        // Instruction encountered
        if c.unwrap().is_alphabetic() {
            let block: Block;

            // Extract entire instruction identifier
            while c.is_some() && c.unwrap().is_alphanumeric() {
                identifier.push(c.unwrap());
                c = next_char(chars)
            }

            let split_block = check_input_type(identifier.as_str(), CalledFrom::WithinBody, chars);
            if split_block.is_some() {
                block = split_block.unwrap();
            } else {
                return vec![];
            }

            blocks.push(block);
        }
        c = next_char(chars);
    }
    blocks
}

pub fn split_file(file_contents: &str) -> Option<Block> {
    let mut chars = file_contents.chars();

    let mut main_block = InstrWithBodyBlock {
        block: "fn".to_string(),
        parameters: vec![Block::Value(ValueBlock {
            block: "main".to_string(),
        })],
        body: Vec::new(),
    };

    let mut c = next_char(&mut chars);

    while c.is_some() {
        let mut identifier = String::new();

        // Instruction encountered
        if c.unwrap().is_alphabetic() {
            let block: Block;

            // Extract entire instruction identifier
            while c.is_some() && c.unwrap().is_alphanumeric() {
                identifier.push(c.unwrap());
                c = next_char(&mut chars)
            }

            let split_block =
                check_input_type(identifier.as_str(), CalledFrom::WithinMain, &mut chars);
            if split_block.is_some() {
                block = split_block.unwrap();
            } else {
                return None;
            }

            main_block.body.push(block);
        }
        c = next_char(&mut chars);
    }

    Some(Block::InstrWithBody(main_block))
}
