use super::frame_type;
use super::instruction;

pub struct InstrWithBodyBlock {
    pub instruction: instruction::InstructionsWithBody,
    pub parameters: Vec<Block>,
    pub body: Vec<Block>,
}

pub struct InstrBlock {
    pub instruction: instruction::Instructions,
    pub parameters: Vec<Block>,
}

pub struct PrimitiveValueBlock {
    pub value: String,
    pub value_type: frame_type::FrameType,
}

pub struct ArrayBlock {
    pub values: Vec<Block>,
}

pub struct StructureBlock {
    pub var_name: String,
    pub var_type: frame_type::FrameType,
    pub var_value: Box<Option<Block>>,
}

pub struct CommentBlock {
    pub value: String,
}

// Base block
pub enum Block {
    InstrWithBody(InstrWithBodyBlock),
    Instr(InstrBlock),
    PrimitiveValue(PrimitiveValueBlock),
    Array(ArrayBlock),
    Structure(StructureBlock),
    Comment(CommentBlock),
}

impl Block {
    pub fn print(&self, indent: i8, indent_char: char) {
        let indent_str = |indent: i8, indent_char: char| {
            let mut str = String::new();

            for i in 0..indent {
                str += indent_char.to_string().as_str();
            }

            str
        };

        match self {
            Block::InstrWithBody(_) => todo!(),
            Block::Instr(blk) => {
                println!("{}Block: Instr", indent_str(indent, indent_char));
                println!("{}Parameters", indent_str(indent + 2, indent_char));

                for i in 0..blk.parameters.len() {
                    blk.parameters[i].print(indent + 2, indent_char);
                }
            }
            Block::PrimitiveValue(pValue) => {
                println!("{}Block: PrimitiveValue", indent_str(indent + 2, indent_char));
                println!("{}Value: {}", indent_str(indent + 4, indent_char), pValue.value);
                println!("{}Type of value: {}", indent_str(indent + 4, indent_char), pValue.value_type.to_string());
            }
            Block::Array(aArray) => {
                println!("{}Block: Array", indent_str(indent + 2, indent_char));
                println!("{}Contents:", indent_str(indent + 2, indent_char));

                for i in 0..aArray.values.len() {
                    aArray.values[i].print(indent + 2, indent_char)
                }
            },
            Block::Structure(_) => todo!(),
            Block::Comment(_) => todo!()
        }
    }
}