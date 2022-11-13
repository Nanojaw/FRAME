pub struct StringBlock {
    Block: String,
    SubBlock: Vec<StringBlock>,
}

fn skip_indent(chars: &mut std::str::Chars) -> Option<char> {
    let mut c = chars.next();
    while c.is_some() && c.unwrap() == ' ' {
        c = chars.next();
    }
    c
}

fn split_instruction<'a>(chars: &mut std::str::Chars, inst_name: String) -> StringBlock {
    let mut block = StringBlock{ Block: inst_name, SubBlock: Vec::new() };

    let mut c = skip_indent(chars);
    if c.is_some() && c.unwrap() == '(' {
        c = skip_indent(chars);

        if c.is_some() && c.unwrap().is_alphabetic() {
            let mut name = String::new();
    
            while c.is_some() && c.unwrap().is_alphanumeric() {
                name.push(c.unwrap());
                c = chars.next();
            }
    
            // Check if we have encountered an instruction
            match name.as_str() {
                "add" => block.SubBlock.push(split_instruction(chars, "add".to_string())),
                _ => block.SubBlock.push( StringBlock {Block: name, SubBlock: Vec::new()})
            }
        }
    }
    block
}

pub fn split_file(str: &str) -> Option<StringBlock> {
    let mut chars = str.chars();

    // Skip indentation
    let mut c = skip_indent(&mut chars);

    // Instructions or variables
    if c.is_some() && c.unwrap().is_alphabetic() {
        let mut name = String::new();

        while c.is_some() && c.unwrap().is_alphanumeric() {
            name.push(c.unwrap());
            c = chars.next();
        }

        // Check if we have encountered an instruction
        match name.as_str() {
            "add" => return Some(split_instruction(&mut chars, "add".to_string())),
            _ => return None
        }
    }
    None
}
