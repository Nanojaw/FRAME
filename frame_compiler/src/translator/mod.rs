use super::splitter::block;
use super::splitter::frame_type;
use super::splitter::instruction;

pub fn to_cpp_file(
    instrs: Vec<block::Block>,
    indent: i8,
    indent_char: char,
) -> Result<String, String> {
    let mut cpp = String::new();

    cpp.push_str("#include \"iostream\"\n");
    //cpp.push_str("#include <vector>\n");
    cpp.push_str("#include <vector>\n\n");

    for i in 0..instrs.len() {
        cpp.push_str(&instrs[i].to_cpp(indent, indent_char, false)?);
    }

    Ok(cpp)
}

impl block::Block {
    pub fn to_cpp(&self, indent: i8, indent_char: char, inside: bool) -> Result<String, String> {
        let indent_str = |indent: i8, indent_char: char| {
            let mut str = String::new();

            for i in 0..indent * 2 {
                str += indent_char.to_string().as_str();
            }

            str
        };

        let mut part_cpp = String::new();

        match self {
            block::Block::Function(func) => {
                let fn_name = &func.name;
                let fn_return_type = &func.return_type;

                if fn_name == &"main".to_string() {
                    part_cpp += "int"
                } else {
                    match fn_return_type {
                        frame_type::FrameType::Array(at) => {
                            part_cpp += "std::vector<";

                            for i in 1..at.dimensions {
                                part_cpp += "std::vector<"
                            }

                            part_cpp += match *at.values_type {
                                frame_type::FrameType::Bool => "bool",
                                frame_type::FrameType::Str => "std::string",
                                frame_type::FrameType::Signed => "long long",
                                frame_type::FrameType::Unsigned => "unsigned long long",
                                frame_type::FrameType::Float => "double",
                                frame_type::FrameType::Variable => todo!(),
                                frame_type::FrameType::Array(_) => todo!(),
                            };

                            for i in 0..at.dimensions {
                                part_cpp += ">"
                            }
                        }

                        _ => {
                            part_cpp += match fn_return_type {
                                frame_type::FrameType::Bool => "bool",
                                frame_type::FrameType::Str => "std::string",
                                frame_type::FrameType::Signed => "long long",
                                frame_type::FrameType::Unsigned => "unsigned long long",
                                frame_type::FrameType::Float => "double",
                                frame_type::FrameType::Variable => todo!(),
                                frame_type::FrameType::Array(_) => todo!(),
                            };
                        }
                    }
                }

                part_cpp += format!(" {}(", fn_name).as_str();

                for i in 0..func.parameters.len() {
                    part_cpp += format!(
                        "{}, ",
                        block::Block::Structure(func.parameters[i].clone()).to_cpp(
                            0,
                            indent_char,
                            inside
                        )?
                    )
                    .as_str();
                }

                if func.parameters.len() != 0 {
                    part_cpp = part_cpp[0..part_cpp.len() - 2].to_string();
                }

                part_cpp += ')'.to_string().as_str();

                part_cpp += " {\n";

                for i in 0..func.body.len() {
                    part_cpp += format!("{}", func.body[i].to_cpp(indent + 1, indent_char, false)?)
                        .as_str();
                }

                part_cpp += "}\n";
            }
            block::Block::InstrWithBody(instr_with_body) => {
                part_cpp += format!(
                    "{}{}(",
                    indent_str(indent, indent_char),
                    instr_with_body.instruction.to_string()
                )
                .as_str();

                if instr_with_body.parameters.len() == 0 {
                    part_cpp += ") {\n";
                } else {
                    part_cpp += format!(
                        "{}) {{\n",
                        instr_with_body.parameters[0].to_cpp(0, indent_char, false)?
                    )
                    .as_str();
                }

                if instr_with_body.instruction == instruction::InstructionsWithBody::Else {
                    part_cpp.remove(part_cpp.find('(').unwrap());
                    part_cpp.remove(part_cpp.find(')').unwrap());
                }

                for i in 0..instr_with_body.body.len() {
                    part_cpp += format!("{}\n", instr_with_body.body[i].to_cpp(indent + 2, indent_char, false)?).as_str();
                }

                part_cpp = part_cpp[0..part_cpp.len() - 1].to_string();

                part_cpp += format!("{}}}\n", indent_str(indent, indent_char)).as_str();
            }
            block::Block::Instr(instr) => match instr.instruction {
                instruction::Instructions::Set => {
                    if instr.parameters.len() > 2 || instr.parameters.len() < 2 {
                        return Err(format!(
                            "Insufficient parameters found in set instr, expected 2 but got {}",
                            instr.parameters.len()
                        ));
                    }

                    let arg1 = instr.parameters[0].to_cpp(indent, indent_char, false)?;
                    let arg2 = instr.parameters[1].to_cpp(0, indent_char, false)?;

                    part_cpp += format!("{} = {};", arg1, arg2).as_str();

                    if part_cpp.contains('\n') {
                        part_cpp.remove(part_cpp.find('\n').unwrap());
                    };

                    part_cpp += '\n'.to_string().as_str();
                }
                instruction::Instructions::Do => {
                    if let block::Block::PrimitiveValue(p_value) = &instr.parameters[0] {
                        part_cpp +=
                            format!("{}{}(", indent_str(indent, indent_char), p_value.value)
                                .as_str();

                        for i in 1..instr.parameters.len() {
                            part_cpp += format!(
                                "{}, ",
                                &instr.parameters[i].to_cpp(0, indent_char, false)?
                            )
                            .as_str();
                        }

                        part_cpp = part_cpp[0..part_cpp.len() - 2].to_string();
                        part_cpp += ")".to_string().as_str();
                    }
                }
                instruction::Instructions::Index => {
                    if instr.parameters.len() > 2 || instr.parameters.len() < 2 {
                        return Err(format!(
                            "Insufficient parameters found in index instr, expected 2 but got {}",
                            instr.parameters.len()
                        ));
                    }

                    let var = instr.parameters[0].to_cpp(indent, indent_char, false)?;
                    let index = instr.parameters[1].to_cpp(0, indent_char, false)?;

                    part_cpp += format!("{}[{}];\n", var, index).as_str();
                }
                instruction::Instructions::Return => {
                    if instr.parameters.len() > 1 || instr.parameters.len() < 1 {
                        return Err(format!(
                            "Insufficient parameters found in return instr, expected 1 but got {}",
                            instr.parameters.len()
                        ));
                    }

                    part_cpp += format!(
                        "{}return {};\n",
                        indent_str(indent, indent_char),
                        instr.parameters[0].to_cpp(0, indent_char, false)?
                    )
                    .as_str();
                }
                instruction::Instructions::Add => {
                    part_cpp += format!(
                        "{}(({}) + ",
                        indent_str(indent, indent_char),
                        instr.parameters[0].to_cpp(0, indent_char, true)?
                    )
                    .as_str();

                    for i in 1..instr.parameters.len() {
                        part_cpp +=
                            format!("({}) + ", instr.parameters[i].to_cpp(0, indent_char, true)?)
                                .as_str();
                    }

                    part_cpp = part_cpp[0..part_cpp.len() - 3].to_string();
                    part_cpp += ')'.to_string().as_str();
                }
                instruction::Instructions::Sub => {
                    part_cpp += format!(
                        "{}(({}) - ",
                        indent_str(indent, indent_char),
                        instr.parameters[0].to_cpp(0, indent_char, true)?
                    )
                    .as_str();

                    for i in 1..instr.parameters.len() {
                        part_cpp +=
                            format!("({}) - ", instr.parameters[i].to_cpp(0, indent_char, true)?)
                                .as_str();
                    }

                    part_cpp = part_cpp[0..part_cpp.len() - 3].to_string();
                    part_cpp += ')'.to_string().as_str();
                }
                instruction::Instructions::Mul => {
                    part_cpp += format!(
                        "{}(({}) * ",
                        indent_str(indent, indent_char),
                        instr.parameters[0].to_cpp(0, indent_char, true)?
                    )
                    .as_str();

                    for i in 1..instr.parameters.len() {
                        part_cpp +=
                            format!("({}) * ", instr.parameters[i].to_cpp(0, indent_char, true)?)
                                .as_str();
                    }

                    part_cpp = part_cpp[0..part_cpp.len() - 3].to_string();
                    part_cpp += ')'.to_string().as_str();
                }
                instruction::Instructions::Div => {
                    part_cpp += format!(
                        "{}(({}) / ",
                        indent_str(indent, indent_char),
                        instr.parameters[0].to_cpp(0, indent_char, true)?
                    )
                    .as_str();

                    for i in 1..instr.parameters.len() {
                        part_cpp +=
                            format!("({}) / ", instr.parameters[i].to_cpp(0, indent_char, true)?)
                                .as_str();
                    }

                    part_cpp = part_cpp[0..part_cpp.len() - 3].to_string();
                    part_cpp += ')'.to_string().as_str();
                }
                instruction::Instructions::Rot => {
                    if instr.parameters.len() > 2 || instr.parameters.len() < 2 {
                        return Err(format!(
                            "Insufficient parameters found in rot instr, expected 2 but got {}",
                            instr.parameters.len()
                        ));
                    }

                    let degree = instr.parameters[0].to_cpp(0, indent_char, false)?;
                    let number = instr.parameters[1].to_cpp(0, indent_char, false)?;

                    part_cpp += format!(
                        "{}std::pow({}, 1/{})",
                        indent_str(indent, indent_char),
                        number,
                        degree
                    )
                    .as_str();
                }
                instruction::Instructions::Eq => {
                    if instr.parameters.len() > 2 || instr.parameters.len() < 2 {
                        return Err(format!(
                            "Insufficient parameters found in eq instr, expected 2 but got {}",
                            instr.parameters.len()
                        ));
                    }

                    let arg1 = instr.parameters[0].to_cpp(0, indent_char, false)?;
                    let arg2 = instr.parameters[1].to_cpp(0, indent_char, false)?;

                    part_cpp +=
                        format!("{}({} == {})", indent_str(indent, indent_char), arg1, arg2)
                            .as_str();
                }
                instruction::Instructions::And => {
                    part_cpp += '('.to_string().as_str();

                    for i in 0..instr.parameters.len() {
                        part_cpp +=
                            format!("{} && ", instr.parameters[i].to_cpp(0, indent_char, false)?)
                                .as_str();
                    }

                    part_cpp = part_cpp[0..part_cpp.len() - 4].to_string();

                    part_cpp += ')'.to_string().as_str();
                }
                instruction::Instructions::Or => {
                    part_cpp += '('.to_string().as_str();

                    for i in 0..instr.parameters.len() {
                        part_cpp +=
                            format!("{} || ", instr.parameters[i].to_cpp(0, indent_char, false)?)
                                .as_str();
                    }

                    part_cpp = part_cpp[0..part_cpp.len() - 4].to_string();

                    part_cpp += ')'.to_string().as_str();
                }
                instruction::Instructions::Not => {
                    if instr.parameters.len() > 1 || instr.parameters.len() < 1 {
                        return Err(format!(
                            "Insufficient parameters found in not instr, expected 1 but got {}",
                            instr.parameters.len()
                        ));
                    }

                    let arg1 = instr.parameters[0].to_cpp(0, indent_char, false)?;

                    part_cpp +=
                        format!("{}(!({}))", indent_str(indent, indent_char), arg1).as_str();
                }
                instruction::Instructions::Lt => {
                    if instr.parameters.len() > 2 || instr.parameters.len() < 2 {
                        return Err(format!("Insufficient parameters found in less than instr, expected 2 but got {}", instr.parameters.len()));
                    }

                    let arg1 = instr.parameters[0].to_cpp(0, indent_char, false)?;
                    let arg2 = instr.parameters[1].to_cpp(0, indent_char, false)?;

                    part_cpp += format!("{}({} < {})", indent_str(indent, indent_char), arg1, arg2)
                        .as_str();
                }
                instruction::Instructions::Gt => {
                    if instr.parameters.len() > 2 || instr.parameters.len() < 2 {
                        return Err(format!("Insufficient parameters found in grater than instr, expected 2 but got {}", instr.parameters.len()));
                    }

                    let arg1 = instr.parameters[0].to_cpp(0, indent_char, false)?;
                    let arg2 = instr.parameters[1].to_cpp(0, indent_char, false)?;

                    part_cpp += format!("{}({} > {})", indent_str(indent, indent_char), arg1, arg2)
                        .as_str();
                }
                instruction::Instructions::Lte => {
                    if instr.parameters.len() > 2 || instr.parameters.len() < 2 {
                        return Err(format!("Insufficient parameters found in less than equals instr, expected 2 but got {}", instr.parameters.len()));
                    }

                    let arg1 = instr.parameters[0].to_cpp(0, indent_char, false)?;
                    let arg2 = instr.parameters[1].to_cpp(0, indent_char, false)?;

                    part_cpp +=
                        format!("{}({} <= {})", indent_str(indent, indent_char), arg1, arg2)
                            .as_str();
                }
                instruction::Instructions::Gte => {
                    if instr.parameters.len() > 2 || instr.parameters.len() < 2 {
                        return Err(format!("Insufficient parameters found in grater than equals instr, expected 2 but got {}", instr.parameters.len()));
                    }

                    let arg1 = instr.parameters[0].to_cpp(0, indent_char, false)?;
                    let arg2 = instr.parameters[1].to_cpp(0, indent_char, false)?;

                    part_cpp +=
                        format!("{}({} >= {})", indent_str(indent, indent_char), arg1, arg2)
                            .as_str();
                }
            },
            block::Block::PrimitiveValue(pValue) => {
                if let frame_type::FrameType::Str = pValue.value_type {
                    part_cpp +=
                        format!("{}\"{}\"", indent_str(indent, indent_char), pValue.value).as_str()
                } else {
                    part_cpp +=
                        format!("{}{}", indent_str(indent, indent_char), pValue.value).as_str()
                }
            }
            block::Block::Array(array) => {
                part_cpp += '{'.to_string().as_str();
                for i in 0..array.values.len() {
                    part_cpp +=
                        format!(" {},", array.values[i].to_cpp(0, indent_char, false)?).as_str();
                }

                part_cpp = part_cpp[0..part_cpp.len() - 1].to_string();

                part_cpp += " }";
            }
            block::Block::Structure(structure) => {
                let sv_name = &structure.var_name;
                let sv_type = &structure.var_type;
                let sv_value = *structure.var_value.clone();

                part_cpp += format!("{}", indent_str(indent, indent_char)).as_str();

                match sv_type {
                    frame_type::FrameType::Array(at) => {
                        part_cpp += "std::vector<";

                        for i in 1..at.dimensions {
                            part_cpp += "std::vector<"
                        }

                        part_cpp += match *at.values_type {
                            frame_type::FrameType::Bool => "bool",
                            frame_type::FrameType::Str => "std::string",
                            frame_type::FrameType::Signed => "long long",
                            frame_type::FrameType::Unsigned => "unsigned long long",
                            frame_type::FrameType::Float => "double",
                            frame_type::FrameType::Variable => todo!(),
                            frame_type::FrameType::Array(_) => todo!(),
                        };

                        for i in 0..at.dimensions {
                            part_cpp += ">"
                        }
                    }

                    _ => {
                        part_cpp += match sv_type {
                            frame_type::FrameType::Bool => "bool",
                            frame_type::FrameType::Str => "std::string",
                            frame_type::FrameType::Signed => "long long",
                            frame_type::FrameType::Unsigned => "unsigned long long",
                            frame_type::FrameType::Float => "double",
                            frame_type::FrameType::Variable => todo!(),
                            frame_type::FrameType::Array(_) => todo!(),
                        };
                    }
                }

                part_cpp += format!(" {}", sv_name).as_str();

                if sv_value.is_some() {
                    part_cpp +=
                        format!(" = {}", sv_value.unwrap().to_cpp(0, indent_char, false)?).as_str();
                }
            }
            block::Block::Comment(comment) => {
                part_cpp +=
                    format!("{}//{}\n", indent_str(indent, indent_char), comment.value).as_str();
            }
        }

        Ok(part_cpp)
    }
}
