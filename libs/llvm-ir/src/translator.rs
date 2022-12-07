use crate::{BasicBlock, Constant, ConstantRef, Function, Instruction, Module, Name, Operand, FPPredicate, IntPredicate, Terminator, Type, TypeRef};

pub enum IrEnum {
    BasicBlock(BasicBlock),
    Constant(Constant),
    ConstantRef(ConstantRef),
    Function(Function),
    Instruction(Instruction),
    Module(Module),
    Name(Name),
    Oprand(Operand),
    FPPredicate(FPPredicate),
    IntPredicate(IntPredicate),
    Terminator(Terminator),
    Type(Type),
    TypeRef(TypeRef)
}