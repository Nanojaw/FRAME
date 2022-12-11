use super::frame_type;
use super::instruction;

pub struct InstrWithBodyBlock {
    pub instruction: instruction::InstructionsWithBody,
    pub parameters: Vec<Block>,
    pub body: Vec<Block>,
}

pub struct InstrBlock {
    pub instruction: instruction::Instructions,
    pub parameters: Vec<Block>,
}

pub struct PrimitiveValueBlock {
    pub value: String,
    pub value_type: frame_type::FrameType,
}

pub struct ArrayBlock {
    pub values: Vec<Block>,
    pub values_type: frame_type::FrameType,
}

pub struct StructureBlock {
    pub var_name: String,
    pub var_type: frame_type::FrameType,
    pub var_value: Box<Option<Block>>,
}

// Base block
pub enum Block {
    InstrWithBody(InstrWithBodyBlock),
    Instr(InstrBlock),
    PrimitiveValue(PrimitiveValueBlock),
    Array(ArrayBlock),
    Structure(StructureBlock),
}
