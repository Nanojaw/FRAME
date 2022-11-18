#[path = "InstrTypeDetails.rs"]
mod InstrTypeDetails;

#[path = "FrameReturnType.rs"]
mod FrameRetrunType;

pub enum BodiedInstrType {
    Fn(InstrTypeDetails::InstrTypeDetails),
    Mod(InstrTypeDetails::InstrTypeDetails),

    If(InstrTypeDetails::TypeDetails),
    Else(InstrTypeDetails::InstrTypeDetails),

    For(InstrTypeDetails::InstrTypeDetails),
    While(InstrTypeDetails::InstrTypeDetails),

}

impl BodiedInstrType {
    pub fn which(name: String) -> BodiedInstrType {
        match name.as_str() {
            "fn" => BodiedInstrType::Fn(InstrTypeDetails::InstrTypeDetails {MinParameters:3, MaxParameters: 3, ReturnType: FrameRetrunType::FrameRetrunType::Void}),
            "if" => BodiedInstrType::If(InstrTypeDetails::InstrTypeDetails {MinParameters:1, MaxParameters: 1, ReturnType: FrameRetrunType::FrameRetrunType::Void}),
            "else" => BodiedInstrType::Else(InstrTypeDetails::InstrTypeDetails {MinParameters:0, MaxParameters: 0, ReturnType: FrameRetrunType::FrameRetrunType::Void}),

            "for" => BodiedInstrType::For(InstrTypeDetails::InstrTypeDetails {MinParameters:3, MaxParameters: 3, ReturnType:FrameRetrunType::FrameReturnType::Void}),
            "while" => BodiedInstrType::While(InstrTypeDetails::InstrTypeDetails {MinParameters:1, MaxParameters: 1, ReturnType:FrameRetrunType::FrameReturnType::Void}),

            "mod" => BodiedInstrType::Mod(InstrTypeDetails {MinParameters: 1, MaxParameters: 1, ReturnType: FrameRetrunType::Void})
        }
    }
}