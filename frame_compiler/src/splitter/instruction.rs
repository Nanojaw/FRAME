pub enum InstructionsWithBody {
    // Flow control instructions
    If,
    ElseIf,
    Else,
}

pub enum Instructions {
    Set,
    Do,
    Index,

    Return,

    // Math instructions
    Add,
    Sub,
    Mul,
    Div,
    Rot,

    // Logic instructions
    Eq,
    Not,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl Instructions {
    pub fn from_str(id: String) -> Result<Self, String> {
        match id.as_str() {
            "set" => Ok(Instructions::Set),
            "do" => Ok(Instructions::Do),
            "index" => Ok(Instructions::Index),

            "return" => Ok(Instructions::Return),

            // Math instructions
            "add" => Ok(Instructions::Add),
            "sub" => Ok(Instructions::Sub),
            "mul" => Ok(Instructions::Mul),
            "div" => Ok(Instructions::Div),
            "rot" => Ok(Instructions::Rot),

            // Logic instructions
            "eq" => Ok(Instructions::Eq),
            "not" => Ok(Instructions::Not),
            "lt" => Ok(Instructions::Lt),
            "gt" => Ok(Instructions::Gt),
            "lte" => Ok(Instructions::Lte),
            "gte" => Ok(Instructions::Gte),

            _ => Err(format!("{} is not a frame instruction", id)),
        }
    }
}
