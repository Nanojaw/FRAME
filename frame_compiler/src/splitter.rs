pub enum StringBlock {
    Block(String),
    SubBlock(Vec<StringBlock>)
}

impl StringBlock {
    fn new(str: &String) {
        let chars = str.chars();
        // TODO: finish
    }
}