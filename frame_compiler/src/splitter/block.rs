use super::frame_type;
use super::instruction;

#[derive(Clone)]
pub struct FunctionBlock {
    pub name: String,
    pub parameters: Vec<StructureBlock>,
    pub body: Vec<Block>,
    pub return_type: frame_type::FrameType,
}

#[derive(Clone)]
pub struct InstrWithBodyBlock {
    pub instruction: instruction::InstructionsWithBody,
    pub parameters: Vec<Block>,
    pub body: Vec<Block>,
}

#[derive(Clone)]
pub struct InstrBlock {
    pub instruction: instruction::Instructions,
    pub parameters: Vec<Block>,
}

#[derive(Clone)]
pub struct PrimitiveValueBlock {
    pub value: String,
    pub value_type: frame_type::FrameType,
}

#[derive(Clone)]
pub struct ArrayBlock {
    pub values: Vec<Block>,
}

#[derive(Clone)]
pub struct StructureBlock {
    pub var_name: String,
    pub var_type: frame_type::FrameType,
    pub var_value: Box<Option<Block>>,
}

#[derive(Clone)]
pub struct CommentBlock {
    pub value: String,
}

// Base block
#[derive(Clone)]
pub enum Block {
    Function(FunctionBlock),
    InstrWithBody(InstrWithBodyBlock),
    Instr(InstrBlock),
    PrimitiveValue(PrimitiveValueBlock),
    Array(ArrayBlock),
    Structure(StructureBlock),
    Comment(CommentBlock),
}