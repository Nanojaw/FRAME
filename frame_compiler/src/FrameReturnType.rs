pub enum FrameRetrunType {
    Bool,
    Number(NumberType),
    Void,
}

pub enum NumberType {
    Signed(i128),
    Unsigned(i128),
    Decimal(i128)
}