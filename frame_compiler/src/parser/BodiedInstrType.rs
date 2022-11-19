use super::InstrTypeDetails;
use super::FrameReturnType::FrameReturnType;

pub enum BodiedInstrType {
    Fn(InstrTypeDetails::InstrTypeDetails),
    Mod(InstrTypeDetails::InstrTypeDetails),

    If(InstrTypeDetails::InstrTypeDetails),
    Else(InstrTypeDetails::InstrTypeDetails),

    For(InstrTypeDetails::InstrTypeDetails),
    While(InstrTypeDetails::InstrTypeDetails),
}

impl BodiedInstrType {
    pub fn which(name: String) -> BodiedInstrType {
        match name.as_str() {
            "fn" => BodiedInstrType::Fn(InstrTypeDetails::InstrTypeDetails {
                MinParameters: 3,
                MaxParameters: 3,
                ReturnType: FrameReturnType::Void,
            }),
            "iff" => BodiedInstrType::If(InstrTypeDetails::InstrTypeDetails {
                MinParameters: 1,
                MaxParameters: 1,
                ReturnType: FrameReturnType::Void,
            }),
            "else" => BodiedInstrType::Else(InstrTypeDetails::InstrTypeDetails {
                MinParameters: 0,
                MaxParameters: 0,
                ReturnType: FrameReturnType::Void,
            }),

            "for" => BodiedInstrType::For(InstrTypeDetails::InstrTypeDetails {
                MinParameters: 3,
                MaxParameters: 3,
                ReturnType: FrameReturnType::Void,
            }),
            "while" => BodiedInstrType::While(InstrTypeDetails::InstrTypeDetails {
                MinParameters: 1,
                MaxParameters: 1,
                ReturnType: FrameReturnType::Void,
            }),

            "mod" => BodiedInstrType::Mod(InstrTypeDetails::InstrTypeDetails {
                MinParameters: 1,
                MaxParameters: 1,
                ReturnType: FrameReturnType::Void,
            }),
        }
    }
}
