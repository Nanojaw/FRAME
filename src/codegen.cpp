#include <map>

#include "AST.hpp"

#include "llvm/IR/IRBuilder.h"
#include "llvm/IR/LLVMContext.h"

static std::unique_ptr<llvm::LLVMContext> TheContext;
static std::unique_ptr<llvm::Module> TheModule;
static std::unique_ptr<llvm::IRBuilder<>> Builder;
static std::map<std::string, llvm::Value*> NamedValues;

std::unique_ptr<AST::Expr> LogError(const char* Str) {
	fprintf(stderr, "Error: %s\n", Str);
	return nullptr;
}

llvm::Value* LogErrorV(const char* Str) {
	LogError(Str);
	return nullptr;
}

llvm::Value* AST::NumberExpr::codegen()
{
	return llvm::ConstantInt::get(*TheContext, llvm::APInt(sizeof(int) * 8, _val, true));
}

llvm::Value* AST::InstructionExpr::codegen()
{
	switch (_instruction)
	{
	case instructions::add: 
		llvm::Value* L = _args[0]->codegen();
		llvm::Value* R = _args[1]->codegen();
		if (!L || !R)
			return nullptr;

		return Builder->CreateAdd(L, R, "addtmp");
	}

	return nullptr;
}
