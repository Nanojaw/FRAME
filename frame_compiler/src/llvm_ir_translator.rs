use crate::parser::ProcessedBlock;
use llvm_ir;

pub fn value_block_to_llvm_ir(
    value_block: &ProcessedBlock::ProcessedValueBlock,
) -> llvm_ir::translator::IrEnum {
    match value_block.value {
        ProcessedBlock::ValueTypes::Bool(bool) => {
            
        }

        ProcessedBlock::ValueTypes::String(str) => todo!(),
        ProcessedBlock::ValueTypes::Number(numType) => todo!(),
        ProcessedBlock::ValueTypes::Variable(var) => todo!(),
    }
}

impl ProcessedBlock::ProcessedBlock {
    pub fn to_llvm_ir(&self) -> llvm_ir::translator::IrEnum {
        match self {
            ProcessedBlock::ProcessedBlock::ProcessedInstrWithBody(InstructionWithBody) => todo!(),
            ProcessedBlock::ProcessedBlock::ProcessedInstr(Instr) => todo!(),

            ProcessedBlock::ProcessedBlock::ProcessedValue(valueBlock) => {
                return value_block_to_llvm_ir(valueBlock);
            }

            ProcessedBlock::ProcessedBlock::ProcessedStructure(Structure) => todo!(),
            ProcessedBlock::ProcessedBlock::ProcessedArray(Array) => todo!(),
        }
    }
}
