use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::AnyTypeEnum;
use inkwell::values::{
    AnyValueEnum, ArrayValue, BasicValueEnum, BasicValueUse, CallSiteValue, CallableValue,
    FloatValue, FunctionValue, GenericValue, GlobalValue, InstructionValue, IntValue,
    MetadataValue, PhiValue, PointerValue, StructValue, VectorValue,
};

use crate::parser::ProcessedBlock::{
    InstrIdentifiers, NumberType, ProcessedArrayBlock, ProcessedBlock, ProcessedStructureBlock,
    ProcessedValueBlock, ValueTypes,
};

fn basic_to_any(value: BasicValueEnum) -> AnyValueEnum {
    match value {
        BasicValueEnum::ArrayValue(value) => AnyValueEnum::ArrayValue(value),
        BasicValueEnum::IntValue(value) => AnyValueEnum::IntValue(value),
        BasicValueEnum::FloatValue(value) => AnyValueEnum::FloatValue(value),
        BasicValueEnum::PointerValue(value) => AnyValueEnum::PointerValue(value),
        BasicValueEnum::StructValue(value) => AnyValueEnum::StructValue(value),
        BasicValueEnum::VectorValue(value) => AnyValueEnum::VectorValue(value),
    }
}

pub struct CodeGen<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub module: Module<'ctx>,

    pub variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    pub fn compile(&mut self, block: &ProcessedBlock) -> Result<AnyValueEnum, String> {
        Err("Yes".to_string())
    }

}
