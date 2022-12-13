use std::fmt::Formatter;
use super::block;

pub struct ArrayType {
    pub dimensions: i8,
    pub values_type: Box<block::Block>,
}

pub enum FrameType {
    Bool,
    Str,
    Signed,
    Unsigned,
    Float,
    Variable,
    Array(ArrayType),
}

impl std::fmt::Display for FrameType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameType::Bool => write!(f, "Bool"),
            FrameType::Str => write!(f, "String"),
            FrameType::Signed => write!(f, "Signed number"),
            FrameType::Unsigned => write!(f, "Unsigned number"),
            FrameType::Float => write!(f, "Double"),
            FrameType::Variable => write!(f, "var"),
            FrameType::Array(_) => write!(f, "Array"),
        }
    }
}