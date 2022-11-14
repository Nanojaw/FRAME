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

fn next_char(chars: &mut Chars) -> Option<char> {
    let mut c = chars.next();

    while c.is_some() && c.unwrap().is_whitespace()  {
        c = chars.next();
    }

    if c.is_some() {
        return c;
    }
    None
}

fn split_params(chars: &mut Chars) -> Vec<Block> {
    let mut c = next_char(chars);

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

                "fn" => params.push(Block::Instr(InstrBlock {
                    block: "fn".to_string(),
                    parameters: split_params(chars),
                })),


                /* #region Conditionals */
                "not" => params.push(Block::Instr(InstrBlock {
                    block: "not".to_string(),
                    parameters: split_params(chars),
                })),

                "eq" => params.push(Block::Instr(InstrBlock {
                    block: "eq".to_string(),
                    parameters: split_params(chars),
                })),
                "neq" => params.push(Block::Instr(InstrBlock {
                    block: "neq".to_string(),
                    parameters: split_params(chars),
                })),
                "greater" => params.push(Block::Instr(InstrBlock {
                    block: "greater".to_string(),
                    parameters: split_params(chars),
                })),
                "less" => params.push(Block::Instr(InstrBlock {
                    block: "less".to_string(),
                    parameters: split_params(chars),
                })),
                
                "and" => params.push(Block::Instr(InstrBlock {
                    block: "and".to_string(),
                    parameters: split_params(chars),
                })),
                "or" => params.push(Block::Instr(InstrBlock {
                    block: "eq".to_string(),
                    parameters: split_params(chars),
                })),
                /* #endregion */

                _ => params.push(Block::Value(ValueBlock {
                    block: alphabetical_type,
                })),
            }

            c = next_char(chars);
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
            c = next_char(chars);
            continue;
        }
    }
    params
}

fn split_body(chars: &mut Chars) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();

    let mut c = next_char(chars);

    if c.is_some() && c.unwrap() == '{' {
        c = next_char(chars)
    }
    else {
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

            // Check what type of instruction we have
            match identifier.as_str() {
                "set" => {
                    block = Block::Instr(InstrBlock {
                        block: "set".to_string(),
                        parameters: split_params(chars),
                    });
                }
                "if" => {
                    block = Block::InstrWithBody(InstrWithBodyBlock {
                        block: "if".to_string(),
                        parameters: split_params(chars),
                        body: split_body(chars),
                    })
                }
                _ => {
                    println!("Unknown instruction \"{identifier}\"");
                    return Vec::new();
                }
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

            // Check what type of instruction we have
            match identifier.as_str() {
                "set" => {
                    block = Block::Instr(InstrBlock {
                        block: "set".to_string(),
                        parameters: split_params(&mut chars),
                    });
                }
                "if" => {
                    block = Block::InstrWithBody(InstrWithBodyBlock {
                        block: "if".to_string(),
                        parameters: split_params(&mut chars),
                        body: split_body(&mut chars),
                    })
                }
                _ => {
                    println!("Unknown instruction \"{identifier}\"");
                    return None;
                }
            }

            main_block.body.push(block);
        }
        c = next_char(&mut chars);
    }
    Some(Block::InstrWithBody(main_block))
}
