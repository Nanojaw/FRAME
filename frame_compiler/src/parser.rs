use crate::splitter::Block;

mod ProcessedBlock;
mod FrameReturnType;
mod BodiedInstrType;
mod InstrTypeDetails;

impl Block {
    pub fn parse(&self) -> Option<ProcessedBlock::ProcessedBlock> {
        match self {
            Block::InstrWithBody(block) => {
                let InstrWithBodyType = BodiedInstrType::BodiedInstrType::which(&block.block);
                
            }
        }
        None
    }
}
