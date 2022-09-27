#ifndef AST_HPP
#define AST_HPP

#include <string>
#include <memory>
#include <vector>

#include "llvm/IR/Constants.h"

enum instructions
{
	comment,
	add
};

class ExprAST {
public:
	virtual ~ExprAST() {}
	virtual llvm::Value* codegen() = 0;
};

class NumberExprAST : public ExprAST {
	int _val;

public:
	NumberExprAST(int val) : _val(val) {}
	llvm::Value* codegen() override;
};

class InstructionExprAST : public ExprAST {
	int _instruction;
	std::vector<std::unique_ptr<ExprAST>> _args;
public:
	InstructionExprAST(const int instruction,
		std::vector<std::unique_ptr<ExprAST>> args)
		: _instruction(instruction), _args(std::move(args)) {}
	llvm::Value* codegen() override;
};

#endif // !AST_HPP