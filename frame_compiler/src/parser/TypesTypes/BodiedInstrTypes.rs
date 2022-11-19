enum BodiedInstrTypes {
    Fn(TypeDetails::TypeDetails),
    Mod(TypeDetails::TypeDetails),

    If(TypeDetails::TypeDetails),
    Else(TypeDetails::TypeDetails),

    For(TypeDetails::TypeDetails),
    While(TypeDetails::TypeDetails),

}

impl BodiedInstrTypes {
    fn which(name: String) -> BodiedInstrTypes {
        match name.as_str() {
            "fn" => BodiedInstrTypes::Fn(),
            "if" => BodiedInstrTypes::If(),
            "else" => BodiedInstrTypes::Else(),

            "for" => BodiedInstrTypes::For(),
            "while" => BodiedInstrTypes::While(),

            "mod" => BodiedInstrTypes::Mod()
        }
    }
}