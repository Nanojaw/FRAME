#[path = "FrameReturnType.rs"]
mod FrameRetrunType;

pub struct InstrTypeDetails {
    pub MinParameters: i8,
    pub MaxParameters: i8,
    pub ReturnType: FrameRetrunType::FrameRetrunType
}