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

pub struct Compiler<'ctx> {
    context: Context,
    builder: Builder<'ctx>,
    module: Module<'ctx>,

    variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(module_name: &str) -> Self {
        let context = Context::create();
        let builder = context.create_builder();
        let module = context.create_module(module_name);
        Compiler {
            context,
            builder,
            module,
            variables: HashMap::new(),
        }
    }

    fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    /*
    fn compile_basic_value(&mut self, block: &ProcessedBlock) -> Result<BasicValueEnum, String> {
        match block {
            ProcessedBlock::ProcessedValue(block) => match block.value {
                ValueTypes::Bool(value) => Ok(BasicValueEnum::IntValue(
                    self.context.bool_type().const_int(value as u64, false),
                )),
                ValueTypes::String(value) => {
                    todo!() // String literals are complex because they need to be allocated somewhere
                            // and might be unicode characters which we also need to handle somehow
                }
                ValueTypes::Number(value_type) => match value_type {
                    NumberType::Signed(value) => Ok(BasicValueEnum::IntValue(
                        self.context.i64_type().const_int(value as u64, false),
                    )),
                    NumberType::Unsigned(value) => Ok(BasicValueEnum::IntValue(
                        self.context.i64_type().const_int(value as u64, true),
                    )),
                    NumberType::Float(value) => Ok(BasicValueEnum::FloatValue(
                        self.context.f64_type().const_float(value),
                    )),
                },
                ValueTypes::Variable(name) => match self.variables.get(&name) {
                    Some(value) => Ok(self.builder.build_load(*value, &name).into()),
                    None => Err(format!("Compiler error: could not find variable {}", name)),
                },
            },
            ProcessedBlock::ProcessedStructure(_) => todo!(),
            ProcessedBlock::ProcessedArray(_) => todo!(),
            _ => Err(format!(
                "Block must be of type Value, Array, or Structure. Got {}",
                block.which()
            )),
        }
    }
*/
 
    pub fn compile(&mut self, block: &ProcessedBlock) -> Result<AnyValueEnum, String> {
        match block {
            ProcessedBlock::ProcessedInstrWithBody(block) => Err("This is not implemented yet".to_string()),
            
            ProcessedBlock::ProcessedInstr(block) => match block.identifier {
                InstrIdentifiers::Set => {
                    if block.parameters.len() != 2 {
                        return Err(format!(
                            "Expected 2 parameters, got {}",
                            block.parameters.len()
                        ));
                    }

                    let var_name: &String = match &block.parameters[0] {
                        ProcessedBlock::ProcessedValue(block) => match &block.value {
                            ValueTypes::Bool(_) => {
                                return Err(format!("First parameter must be a variable, got bool"))
                            }
                            ValueTypes::String(_) => {
                                return Err(format!(
                                    "First parameter must be a variable, got string"
                                ))
                            }
                            ValueTypes::Number(_) => {
                                return Err(format!(
                                    "First parameter must be a variable, got number"
                                ))
                            }
                            ValueTypes::Variable(var) => var,
                        },
                        _ => return Err(format!("First parameter must be a value")),
                    };

                    // Propagate the error
                    let value = self.compile_basic_value(&block.parameters[1])?;
                    
                    if !self.variables.contains_key(var_name) {
                        let ptr = self.builder.build_alloca(value.get_type(), var_name);
                        self.builder.build_store(ptr, value);
                        self.variables.insert(*var_name, ptr);
                    }


                    return Err(format!("todo"));
                },

                _ => return Err(format!("Instr is not implemented"))
            },
            
            ProcessedBlock::ProcessedValue(block) => match block.value {
                ValueTypes::Bool(value) => Ok(BasicValueEnum::IntValue(
                    self.context.bool_type().const_int(value as u64, false),
                )),
                ValueTypes::String(value) => {
                    todo!() // String literals are complex because they need to be allocated somewhere
                            // and might be unicode characters which we also need to handle somehow
                }
                ValueTypes::Number(value_type) => match value_type {
                    NumberType::Signed(value) => Ok(BasicValueEnum::IntValue(
                        self.context.i64_type().const_int(value as u64, false),
                    )),
                    NumberType::Unsigned(value) => Ok(BasicValueEnum::IntValue(
                        self.context.i64_type().const_int(value as u64, true),
                    )),
                    NumberType::Float(value) => Ok(BasicValueEnum::FloatValue(
                        self.context.f64_type().const_float(value),
                    )),
                },
                ValueTypes::Variable(name) => match self.variables.get(&name) {
                    Some(value) => Ok(self.builder.build_load(*value, &name).into()),
                    None => Err(format!("Compiler error: could not find variable {}", name)),
                },
            },
            ProcessedBlock::ProcessedStructure(_) => todo!(),

            ProcessedBlock::ProcessedArray(_) => todo!(),

            _ => Err(format!(
                "Block must be of type InstrWithBody, Instr, Value, Array, or Structure. Got {}",
                block.which()
            )),








            ProcessedBlock::ProcessedArray(block) => Ok(basic_to_any(self.compile_basic_value(ProcessedBlock::ProcessedArray(block))?)),
            ProcessedBlock::ProcessedStructure(block) => Ok(basic_to_any(self.compile_basic_value(ProcessedBlock::ProcessedStructure(block))?)),
        }
    }
}
