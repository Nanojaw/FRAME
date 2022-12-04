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
    pub fn which(id: &String) -> Self {
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

            _ => panic!("lol"),
        }
    }
}

pub struct ProcessedInstrWithBodyBlock {
    pub identifier: InstrIdentifiers,
    pub parameters: Vec<ProcessedBlock>,
    pub body: Vec<ProcessedBlock>,
}

pub struct ProcessedInstrBlock {
    pub identifier: InstrIdentifiers,
    pub parameters: Vec<ProcessedBlock>,
}

pub enum NumberType {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
}

pub enum ValueTypes {
    Bool(bool),
    String(String),
    Number(NumberType),
    Variable(String),
}

pub struct ProcessedValueBlock {
    pub value: ValueTypes,
}

impl ProcessedValueBlock {
    pub fn which(&self) -> String {
        match self.value {
            ValueTypes::Bool(_) => "B".to_string(),
            ValueTypes::String(_) => "S".to_string(),
            ValueTypes::Number(_) => "N".to_string(),
            ValueTypes::Variable(_) => "V".to_string(),
        }
    }
}

pub struct ProcessedStructureBlock {
    pub entries: Vec<ProcessedStructureEntry>,
}

pub enum GenericTypes {
    Bool,
    String,
    Number,
    Variable,
    Array,
    Structure,
}

impl GenericTypes {
    pub fn which(id: &String) -> Self {
        match id.as_str() {
            "bool" => GenericTypes::Bool,
            "string" => GenericTypes::String,
            "number" => GenericTypes::Number,
            "variable" => GenericTypes::Variable,
            "array" => GenericTypes::Array,
            "structure" => GenericTypes::Structure,
            _ => panic!("lol"),
        }
    }
}

pub struct ProcessedStructureEntry {
    pub variable_string: String,
    pub variable_type: GenericTypes,
    pub value: Option<ProcessedBlock>,
}

pub struct ProcessedArrayBlock {
    pub entries: Vec<ProcessedBlock>,
    pub t: GenericTypes,
}

pub enum ProcessedBlock {
    ProcessedInstrWithBody(ProcessedInstrWithBodyBlock),
    ProcessedInstr(ProcessedInstrBlock),
    ProcessedValue(ProcessedValueBlock),
    ProcessedStructure(ProcessedStructureBlock),
    ProcessedArray(ProcessedArrayBlock),
}

impl ProcessedBlock {
    pub fn which(&self) -> String {
        match self {
            ProcessedBlock::ProcessedInstrWithBody(_) => "PIWB".to_string(),
            ProcessedBlock::ProcessedInstr(_) => "PIB".to_string(),
            ProcessedBlock::ProcessedValue(_) => "PVB".to_string(),
            ProcessedBlock::ProcessedStructure(_) => "PSB".to_string(),
            ProcessedBlock::ProcessedArray(_) => "PAB".to_string(),
        }
    }
}
