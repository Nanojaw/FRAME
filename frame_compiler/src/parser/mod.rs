use crate::splitter::Block;
mod TypeDetails;


impl Block {
    pub fn parse(&self) -> Option<ProcessedBlock> {
        match self {
            Block::InstrWithBody(block) => {
                None
            }
        }
    }
}
