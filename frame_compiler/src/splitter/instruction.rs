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
