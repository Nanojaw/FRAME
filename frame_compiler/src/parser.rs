use crate::splitter::*;

mod ProcessedBlock;
use ProcessedBlock::*;

impl Block {
    pub fn parse(&self) -> Option<ProcessedBlock::ProcessedBlock> {
        match self {
            Block::InstrWithBody(block) => {
                let instr_name: String = block.block;

                let parameters: Vec<ProcessedBlock::ProcessedBlock> = vec![];
                for i in 0..block.parameters.len() {
                    parameters.push(block.parameters[i].parse().unwrap());
                }

                let body: Vec<ProcessedBlock::ProcessedBlock> = vec![];
                for i in 0..block.body.len() {
                    body.push(block.body[i].parse().unwrap());
                }

                return Some(ProcessedBlock::ProcessedBlock::ProcessedInstrWithBody(ProcessedInstrWithBodyBlock { identifier: InstrIdentifiers::which(instr_name) }));
            }
        }
        None
    }
}
