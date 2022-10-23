#include "AST.hpp"

#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LLVMContext.h"

#include <map>

const static std::unique_ptr<llvm::LLVMContext> the_context;
const static std::unique_ptr<llvm::Module> the_module;
const static std::unique_ptr<llvm::IRBuilder<>> builder;
const static std::map<std::string, llvm::Value*> named_values;

auto LogError(const char* str) -> std::unique_ptr<AST::Expr> {
  fprintf(stderr, "Codegen error: %s\n", str);
  return nullptr;
}

auto LogErrorV(const char* str) -> llvm::Value* {
  LogError(str);
  return nullptr;
}

auto AST::NumberExpr::codegen() -> llvm::Value* {
  return llvm::ConstantInt::get(*the_context, llvm::APInt(sizeof(int) * 8, _val, true));
}

auto AST::InstructionExpr::codegen() -> llvm::Value* {
  if (_instruction == instructions::add) {
    llvm::Value* l = _args[0]->codegen();
    llvm::Value* r = _args[1]->codegen();
    if (!l || !r) return nullptr;

    return builder->CreateAdd(l, r, "addtmp");
  } else {
    return nullptr;
  }
}
