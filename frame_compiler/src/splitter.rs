pub struct StringBlock {
    block: String,
    sub_block: Vec<StringBlock>,
}

impl StringBlock {
    pub fn print(&self, indent: i32) {
        let indent_str = (0..indent).map(|_| " ").collect::<String>();
        println!("{}Block: {} {{", indent_str, self.block);
        print!("{} SubBlocks: ", indent_str);
        
        if self.sub_block.len() == 0 {
            println!("Empty");
            return;
        } else {
            println!();
        }

        for i in 0..self.sub_block.len() {
            self.sub_block[i].print(indent+2)
        }
        
        println!("{}}}", indent_str);
    }
}

fn skip_indent(
    chars: &mut std::str::Chars,
    advance: bool,
    curr_char: Option<char>,
) -> Option<char> {
    if advance {
        let mut c = chars.next();

        while c.is_some() && c.unwrap() == ' ' {
            c = chars.next()
        }

        c
    } else {
        let mut c = curr_char;
        while c.is_some() && c.unwrap() == ' ' {
            c = chars.next()
        }

        c
    }
}

fn split_instruction<'a>(
    chars: &mut std::str::Chars,
    inst_name: String,
    current_char: Option<char>,
) -> StringBlock {
    let mut block = StringBlock {
        block: inst_name,
        sub_block: Vec::new(),
    };

    let mut c = skip_indent(chars, false, current_char);

    if c.is_some() && c.unwrap() == '(' {
        while c.is_some() && c.unwrap() != ')' {
            c = skip_indent(chars, true, None);

            if c.is_some() && c.unwrap().is_alphabetic() {
                let mut name = String::new();

                while c.is_some() && c.unwrap().is_alphanumeric() {
                    name.push(c.unwrap());
                    c = chars.next();
                }

                // Check if we have encountered an instruction
                match name.as_str() {
                    "add" => block
                        .sub_block
                        .push(split_instruction(chars, "add".to_string(), c)),
                    "sub" => block
                        .sub_block
                        .push(split_instruction(chars, "sub".to_string(), c)),
                        _ => panic!("{name} is not implemented yet"),
                }
            } else if c.is_some() && c.unwrap().is_digit(10) {
                let mut digit_str = String::new();

                while c.is_some() && (c.unwrap().is_digit(10) || c.unwrap() == '.') {
                    digit_str.push(c.unwrap());
                    c = chars.next();
                }

                block.sub_block.push(StringBlock {
                    block: digit_str,
                    sub_block: Vec::new(),
                })
            }
        }
    }

    block
}

pub fn split_file(str: &str, file_name: &str) -> Option<StringBlock> {
    let mut chars = str.chars();

    // Every file has a hidden top level module
    let mut top_mod = StringBlock {
        block: file_name.to_string(),
        sub_block: Vec::new(),
    };

    // Skip indentation
    let mut c = skip_indent(&mut chars, true, None);

    // Instructions or variables
    if c.is_some() && c.unwrap().is_alphabetic() {
        let mut name = String::new();

        while c.is_some() && c.unwrap().is_alphanumeric() {
            name.push(c.unwrap());
            c = chars.next();
        }

        // Check if we have encountered an instruction
        match name.as_str() {
            "add" => top_mod
                .sub_block
                .push(split_instruction(&mut chars, "add".to_string(), c)),
            "sub" => top_mod
                .sub_block
                .push(split_instruction(&mut chars, "sub".to_string(), c)),
            _ => panic!("{name} is not implemented yet"),
        }
    }

    Some(top_mod)
}
