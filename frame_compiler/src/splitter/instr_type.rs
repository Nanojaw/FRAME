pub enum InstrType {
    Regular,
    WithBody,
}

pub fn toInstrType(instr_id: &String) -> Result<InstrType, String> {
    match instr_id.as_str() {
        "if" => Ok(InstrType::WithBody),
        "elseif" => Ok(InstrType::WithBody),
        "else" => Ok(InstrType::WithBody),

        "set" => Ok(InstrType::Regular),
        "do" => Ok(InstrType::Regular),
        "index" => Ok(InstrType::Regular),

        "return" => Ok(InstrType::Regular),

        "add" => Ok(InstrType::Regular),
        "sub" => Ok(InstrType::Regular),
        "mul" => Ok(InstrType::Regular),
        "div" => Ok(InstrType::Regular),
        "rot" => Ok(InstrType::Regular),
        "eq" => Ok(InstrType::Regular),
        "not" => Ok(InstrType::Regular),
        "lt" => Ok(InstrType::Regular),
        "gt" => Ok(InstrType::Regular),
        "lte" => Ok(InstrType::Regular),
        "gte" => Ok(InstrType::Regular),

        _ => Err(format!(
            "{} is a variable, variables are not permitted selfstanding in file",
            instr_id
        )),
    }
}
