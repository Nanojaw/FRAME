use std::str::Chars;

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

pub enum Block {
    Value(ValueBlock),
    Instr(InstrBlock),
    InstrWithBody(InstrWithBodyBlock),
}

impl Block {
    pub fn print(&self, indent: i32) {
        let indent_str = (0..indent).map(|_| " ").collect::<String>();
        match self {
            Block::Value(block) => {
                println!("{}Block: {}", indent_str, block.block)
            }
            Block::Instr(block) => {
                println!("{}Block: {}", indent_str, block.block);
                print!("{}Params: ", indent_str);

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
                print!("{}Params: ", indent_str);

                if block.parameters.len() == 0 {
                    println!("Empty");
                    return;
                } else {
                    println!();
                }

                for i in 0..block.parameters.len() {
                    block.parameters[i].print(indent + 2)
                }

                print!("{}Body: ", indent_str);

                if block.body.len() == 0 {
                    println!("Empty");
                    return;
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

fn NextChar(chars: &mut Chars) -> Option<char> {
    let mut c = chars.next();

    while c.is_some() && c.unwrap() == ' ' {
        c = chars.next();
    }

    if c.is_some() {
        return c;
    }
    None
}

fn split_params(chars: &mut Chars) -> Vec<Block> {
    let mut c = NextChar(chars);

    let mut params: Vec<Block> = Vec::new();

    while c.is_some() && c.unwrap() != ')' {
        if c.unwrap().is_alphabetic() {
            let mut alphabetical_type = String::new();

            while c.is_some() && c.unwrap().is_alphanumeric() {
                alphabetical_type.push(c.unwrap());
                c = chars.next();
            }

            match alphabetical_type.as_str() {
                "add" => params.push(Block::Instr(InstrBlock {
                    block: "add".to_string(),
                    parameters: split_params(chars),
                })),
                _ => params.push(Block::Value(ValueBlock {
                    block: alphabetical_type,
                })),
            }

            c = NextChar(chars);
        }

        if c.unwrap().is_digit(10) {
            let mut number_str = String::new();

            while c.is_some() && (c.unwrap().is_digit(10) || c.unwrap() == '.') {
                number_str.push(c.unwrap());
                c = chars.next();
            }

            params.push(Block::Value(ValueBlock { block: number_str }))
        }

        if c.unwrap() == ',' {
            c = NextChar(chars);
            continue;
        }
    }
    params
}

pub fn split_file(file_contents: &str, filename: &str) -> Option<Block> {
    let mut chars = file_contents.chars();

    let mut main_block = InstrWithBodyBlock {
        block: "fn".to_string(),
        parameters: vec![Block::Value(ValueBlock {
            block: "main".to_string(),
        })],
        body: Vec::new(),
    };

    let mut c = NextChar(&mut chars);

    while c.is_some() {
        let mut identifier = String::new();

        // Instruction encountered
        if c.unwrap().is_alphabetic() {
            let mut block: Block;

            // Extract entire instruction identifier
            while c.is_some() && c.unwrap().is_alphanumeric() {
                identifier.push(c.unwrap());
                c = NextChar(&mut chars)
            }

            // Check what type of instruction we have
            match identifier.as_str() {
                "add" => {
                    block = Block::Instr(InstrBlock {
                        block: "add".to_string(),
                        parameters: split_params(&mut chars),
                    });
                }
                _ => {
                    println!("Unknown instruction \"{identifier}\"");
                    return None;
                }
            }

            main_block.body.push(block);
        }
        c = NextChar(&mut chars);
    }
    Some(Block::InstrWithBody(main_block))
}
