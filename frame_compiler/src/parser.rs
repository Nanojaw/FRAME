use crate::splitter::*;

mod ProcessedBlock;
use ProcessedBlock::*;

impl Block {
    pub fn parse(&self) -> Option<ProcessedBlock::ProcessedBlock> {
        match self {
            Block::InstrWithBody(block) => {
                let instr_identifier = InstrIdentifiers::which(&block.block);

                let mut parameters: Vec<ProcessedBlock::ProcessedBlock> = vec![];
                for i in 0..block.parameters.len() {
                    parameters.push(block.parameters[i].parse().unwrap());
                }

                let mut body: Vec<ProcessedBlock::ProcessedBlock> = vec![];
                for i in 0..block.body.len() {
                    body.push(block.body[i].parse().unwrap());
                }

                return Some(ProcessedBlock::ProcessedBlock::ProcessedInstrWithBody(
                    ProcessedInstrWithBodyBlock {
                        identifier: instr_identifier,
                        parameters: parameters,
                        body: body,
                    },
                ));
            }
            Block::Instr(block) => {
                let instr_identifier = InstrIdentifiers::which(&block.block);

                let mut parameters: Vec<ProcessedBlock::ProcessedBlock> = vec![];
                for i in 0..block.parameters.len() {
                    parameters.push(block.parameters[i].parse().unwrap());
                }

                return Some(ProcessedBlock::ProcessedBlock::ProcessedInstr(
                    ProcessedInstrBlock {
                        identifier: instr_identifier,
                        parameters: parameters,
                    },
                ));
            }
            Block::Value(block) => {
                if block.is_string {
                    return Some(ProcessedBlock::ProcessedBlock::ProcessedValue(
                        ProcessedValueBlock {
                            value: ProcessedBlock::ValueTypes::String(block.block.clone()),
                        },
                    ));
                } else if block.block.contains('.') {
                    return Some(ProcessedBlock::ProcessedBlock::ProcessedValue(
                        ProcessedValueBlock {
                            value: ProcessedBlock::ValueTypes::Number(NumberType::Float(
                                block.block.parse::<f64>().unwrap(),
                            )),
                        },
                    ));
                } else if block.block.starts_with('-') || block.block.as_bytes()[0].is_ascii_digit()
                {
                    if block.block.starts_with('-') {
                        return Some(ProcessedBlock::ProcessedBlock::ProcessedValue(
                            ProcessedValueBlock {
                                value: ProcessedBlock::ValueTypes::Number(NumberType::Signed(
                                    block.block.parse::<i128>().unwrap(),
                                )),
                            },
                        ));
                    } else {
                        return Some(ProcessedBlock::ProcessedBlock::ProcessedValue(
                            ProcessedValueBlock {
                                value: ProcessedBlock::ValueTypes::Number(NumberType::Unsigned(
                                    block.block.parse::<u128>().unwrap(),
                                )),
                            },
                        ));
                    }
                } else {
                    if block.block == "true" || block.block == "false" {
                        match block.block.as_str() {
                            "true" => Some(ProcessedBlock::ProcessedBlock::ProcessedValue(
                                ProcessedValueBlock {
                                    value: ProcessedBlock::ValueTypes::Bool(true),
                                },
                            )),
                            "false" => Some(ProcessedBlock::ProcessedBlock::ProcessedValue(
                                ProcessedValueBlock {
                                    value: ProcessedBlock::ValueTypes::Bool(false),
                                },
                            )),

                            _ => panic!("lol")
                        }
                    } else {
                        return Some(ProcessedBlock::ProcessedBlock::ProcessedValue(
                            ProcessedValueBlock {
                                value: ProcessedBlock::ValueTypes::Variable(block.block.clone()),
                            },
                        ));
                    }
                }
            }
            Block::Structure(block) => {
                let mut entries: Vec<ProcessedBlock::ProcessedStructureEntry> = vec![];

                for i in 0..block.entries.len() {
                    let variable_name: String = (&block.entries[i].var_name).clone();
                    let variable_type: GenericTypes =
                        GenericTypes::which(&block.entries[i].frame_type);

                    let value: Option<ProcessedBlock::ProcessedBlock>;
                    if block.entries[i].value.is_some() {
                        value = ((&block.entries[i].value).clone())
                            .as_ref()
                            .unwrap()
                            .parse();
                    } else {
                        value = None;
                    }

                    let entry = ProcessedBlock::ProcessedStructureEntry {
                        variable_string: variable_name,
                        variable_type: variable_type,
                        value: value,
                    };
                    entries.push(entry);
                }

                return Some(ProcessedBlock::ProcessedBlock::ProcessedStructure(
                    ProcessedStructureBlock { entries: entries },
                ));
            }
            Block::Array(block) => {
                let mut values: Vec<ProcessedBlock::ProcessedBlock> = vec![];
                for i in 0..block.values.len() {
                    values.push(block.values[i].parse().unwrap());
                }

                let pbt = values[0].which();

                for i in 0..values.len() {
                    if values[i].which() != pbt {
                        panic!("The elements in the array do not have the same type")
                    }
                }

                match &values[0] {
                    ProcessedBlock::ProcessedBlock::ProcessedValue(block) => {
                        let pvt = block.which();
                        
                        for i in 0..values.len() {
                            match &values[i] {
                                ProcessedBlock::ProcessedBlock::ProcessedValue(huh) => {
                                    if huh.which() != pvt {
                                        panic!("The value types are not the same")
                                    } 
                                },
                                _ => panic!("huh")
                            }
                        }
                    },
                    _ => panic!("huh")
                }

                return Some(ProcessedBlock::ProcessedBlock::ProcessedArray(
                    ProcessedArrayBlock { entries: values, t: GenericTypes::Number },
                ));
            }
        }
    }
}
