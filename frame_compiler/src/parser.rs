use crate::splitter::Block;

#[path = "ProcessedBlock.rs"]
mod ProcessedBlock;

#[path = "BodiedInstrType.rs"]
mod BodiedInstrType;

impl Block {
    pub fn parse(&self) -> Option<ProcessedBlock::ProcessedBlock> {
        match self {
            Block::InstrWithBody(block) => {
                let InstrWithBodyType = BodiedInstrType::BodiedInstrType::which(block.block);
            }
        }
        None
    }
}
