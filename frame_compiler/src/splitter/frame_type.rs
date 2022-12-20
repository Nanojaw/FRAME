use std::fmt::Formatter;
use super::block;

#[derive(Clone)]
pub struct ArrayType {
    pub dimensions: i8,
    pub values_type: Box<FrameType>,
}

#[derive(Clone)]
pub enum FrameType {
    Bool,
    Str,
    Signed,
    Unsigned,
    Float,
    Variable,
    Array(ArrayType),
}

impl FrameType {
    pub fn which(id: String) -> Result<Self, String> {
        match id.as_str() {
            "bool" => Ok(FrameType::Bool),
            "str" => Ok(FrameType::Str),
            "signed" => Ok(FrameType::Signed),
            "unsigned" => Ok(FrameType::Unsigned),
            "float" => Ok(FrameType::Float),
            "var" => Ok(FrameType::Variable),
            _ => Err("Prolly an array".to_string())
        }
    }
}

impl std::fmt::Display for FrameType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FrameType::Bool => write!(f, "Bool"),
            FrameType::Str => write!(f, "String"),
            FrameType::Signed => write!(f, "Signed number"),
            FrameType::Unsigned => write!(f, "Unsigned number"),
            FrameType::Float => write!(f, "Double"),
            FrameType::Variable => write!(f, "Var"),
            FrameType::Array(_) => write!(f, "Array"),
        }
    }
}