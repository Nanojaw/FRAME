use super::block;

pub struct ArrayType {
    pub dimensions: i8,
    pub values_type: Box<block::Block>,
}

pub enum FrameType {
    Bool,
    Str,
    Char,
    Signed,
    Unsigned,
    Float,
    Array(ArrayType),
}
