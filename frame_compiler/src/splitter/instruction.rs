#[derive(Clone, PartialEq)]
pub enum InstructionsWithBody {
    // Flow control instructions
    Fn,
    If,
    ElseIf,
    Else,
}

impl std::fmt::Display for InstructionsWithBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionsWithBody::Fn => write!(f, "{}", "fn"),
            InstructionsWithBody::If => write!(f, "{}", "if"),
            InstructionsWithBody::ElseIf => write!(f, "{}", "elseif"),
            InstructionsWithBody::Else => write!(f, "{}", "else"),
        }
    }
}

impl InstructionsWithBody {
    pub fn from_str(id: String) -> Result<Self, String> {
        match id.as_str() {
            "fn" => Ok(InstructionsWithBody::Fn),
            "if" => Ok(InstructionsWithBody::If),
            "elseif" => Ok(InstructionsWithBody::ElseIf),
            "else" => Ok(InstructionsWithBody::Else),

            _ => Err(format!("{} is not a frame instruction", id)),
        }
    }
}


#[derive(Clone)]
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
    And,
    Or,
    Not,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl std::fmt::Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::Set => write!(f, "{}", "set"),
            Instructions::Do => write!(f, "{}", "do"),
            Instructions::Index => write!(f, "{}", "index"),
            Instructions::Return => write!(f, "{}", "return"),
            Instructions::Add => write!(f, "{}", "add"),
            Instructions::Sub => write!(f, "{}", "sub"),
            Instructions::Mul => write!(f, "{}", "mul"),
            Instructions::Div => write!(f, "{}", "div"),
            Instructions::Rot => write!(f, "{}", "rot"),
            Instructions::Eq => write!(f, "{}", "eq"),
            Instructions::And => write!(f, "{}", "and"),
            Instructions::Or => write!(f, "{}", "or"),
            Instructions::Not => write!(f, "{}", "not"),
            Instructions::Lt => write!(f, "{}", "lt"),
            Instructions::Gt => write!(f, "{}", "gt"),
            Instructions::Lte => write!(f, "{}", "lte"),
            Instructions::Gte => write!(f, "{}", "gte"),
        }
    }
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
            "and" => Ok(Instructions::And),
            "or" => Ok(Instructions::Or),
            "not" => Ok(Instructions::Not),
            "lt" => Ok(Instructions::Lt),
            "gt" => Ok(Instructions::Gt),
            "lte" => Ok(Instructions::Lte),
            "gte" => Ok(Instructions::Gte),

            _ => Err(format!("{} is not a frame instruction", id)),
        }
    }
}
