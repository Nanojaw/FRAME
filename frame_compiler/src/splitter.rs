pub struct StringBlock {
    block: String,
    sub_block: Vec<StringBlock>,
}

fn skip_indent(chars: &mut std::str::Chars, advance: bool, curr_char: Option<char>) -> Option<char> {
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

fn split_instruction<'a>(chars: &mut std::str::Chars, inst_name: String, current_char: Option<char>) -> StringBlock {
    let mut block = StringBlock{ block: inst_name, sub_block: Vec::new() };

    let mut c = skip_indent(chars, false, current_char);

    if c.is_some() && c.unwrap() == '(' {
        c = skip_indent(chars, true, None);

        if c.is_some() && c.unwrap().is_alphabetic() {
            let mut name = String::new();
    
            while c.is_some() && c.unwrap().is_alphanumeric() {
                name.push(c.unwrap());
                c = chars.next();
            }
    
            // Check if we have encountered an instruction
            match name.as_str() {
                "add" => block.sub_block.push(split_instruction(chars, "add".to_string(), c)),
                "sub" => block.sub_block.push(split_instruction(chars, "sub".to_string(), c)),
                _ => block.sub_block.push( StringBlock {block: name, sub_block: Vec::new()})
            }
        } else if c.is_some() && c.unwrap().is_digit(10) {
            let mut digit_str = String::new();
    
            while c.is_some() && (c.unwrap() != ')' && c.unwrap() != ',') {
                digit_str.push(c.unwrap());
                c = chars.next();
            }
    
            block.sub_block.push(StringBlock {block: digit_str, sub_block: Vec::new()})
        }
    }

    match c.unwrap() {
        ',' => {
            block.sub_block.push(slpit_generic(chars))
            
        },
        ')' => {
            return block;
        },
        _ => panic!()
    }

    println!("{}", "huh");

    block
}

pub fn split_file(str: &str) -> Vec<Option<StringBlock>> {
    let mut chars = str.chars();

    let mut stuff: Vec<Option<StringBlock>> = Vec::new();

    while true {
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
                "add" => stuff.push(Some(split_instruction(&mut chars, "add".to_string(), c))),
                "sub" => stuff.push(Some(split_instruction(&mut chars, "sub".to_string(), c))),
                _ => stuff.push(None)
            }
        }
        return stuff;
    }

    stuff

    
}
