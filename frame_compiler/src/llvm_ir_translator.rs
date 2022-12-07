use std::sync::Arc;

use crate::parser::ProcessedBlock;
use llvm_ir::{self, ConstantRef};

pub fn value_block_to_llvm_ir(
    value_block: &ProcessedBlock::ProcessedValueBlock,
) -> Option<llvm_ir::translator::IrEnum> {
    match &value_block.value {
        ProcessedBlock::ValueTypes::Bool(bool) => {
            return Some(llvm_ir::translator::IrEnum::Constant(llvm_ir::Constant::Int {
                bits: 8,
                value: bool.clone() as u64,
            }))
        }

        ProcessedBlock::ValueTypes::String(str) => {
            if str.is_ascii() != true { panic!("String is not ascii") }

            let str_bytes = str.as_bytes();

            let huh_arc: Arc<llvm_ir::Type> = Arc::new(llvm_ir::Type::IntegerType { bits: 64 });
            let llvm_ir_type = llvm_ir::TypeRef(huh_arc);
            
            
            
            
            
            
            
            let mut bytes_in_llvm_ir: Vec<ConstantRef> = vec![];

            let llvm_str_array = llvm_ir::Constant::Array { element_type: llvm_ir_type, elements: bytes_in_llvm_ir }
            
            None
        },

        ProcessedBlock::ValueTypes::Number(num_type) => match &num_type {
            ProcessedBlock::NumberType::Signed(value) => Some(llvm_ir::translator::IrEnum::Constant(llvm_ir::Constant::Int { bits: 64, value: value.clone().abs() as u64 })),
            ProcessedBlock::NumberType::Unsigned(value) => None,
            ProcessedBlock::NumberType::Float(value) => Some(llvm_ir::translator::IrEnum::Constant(llvm_ir::Constant::Float(llvm_ir::constant::Float::Double(value.clone())))),
        },

        ProcessedBlock::ValueTypes::Variable(var) => Some(llvm_ir::translator::IrEnum::Name(llvm_ir::Name::Name(Box::new(var.clone())))),
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
