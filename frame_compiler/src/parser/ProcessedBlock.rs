pub enum InstrIdentifiers {
    Set,
    Do,
    Index,
    Fn,

    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Rot,

    If,
    Else,
    For,

    Eq,
    Not,
    And,
    Or,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl InstrIdentifiers {
    pub fn which(id: String) -> Self {
        match id.as_str() {
            "set" => InstrIdentifiers::Set,
            "do" => InstrIdentifiers::Do,
            "index" => InstrIdentifiers::Index,
            "fn" => InstrIdentifiers::Fn,
            "add" => InstrIdentifiers::Add,
            "sub" => InstrIdentifiers::Sub,
            "mul" => InstrIdentifiers::Mul,
            "div" => InstrIdentifiers::Div,
            "pow" => InstrIdentifiers::Pow,
            "rot" => InstrIdentifiers::Rot,
            "if" => InstrIdentifiers::If,
            "else" => InstrIdentifiers::Else,
            "for" => InstrIdentifiers::For,
            "eq" => InstrIdentifiers::Eq,
            "not" => InstrIdentifiers::Not,
            "and" => InstrIdentifiers::And,
            "or" => InstrIdentifiers::Or,
            "lt" => InstrIdentifiers::Lt,
            "gt" => InstrIdentifiers::Gt,
            "lte" => InstrIdentifiers::Lte,
            "gte" => InstrIdentifiers::Gte,
        }
    }
}

pub struct ProcessedInstrWithBodyBlock {
    pub identifier: InstrIdentifiers,
    pub parameters: Vec<ProcessedBlock>,
    pub body: Vec<ProcessedBlock>,
}

pub enum ProcessedBlock {
    ProcessedInstrWithBody(ProcessedInstrWithBodyBlock),
}
