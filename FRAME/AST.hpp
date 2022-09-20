#ifndef AST_HPP
#define AST_HPP

#include <string>
#include <memory>
#include <vector>

class ExprAST {
public:
	virtual ~ExprAST() {}
};

class NumberExprAST : public ExprAST {
	int _val;
public:
	NumberExprAST(int val) : _val(val) {};
};

class InstructionExprAST : public ExprAST {
	std::string _name;
	std::vector<std::unique_ptr<ExprAST>> _args;
public:
	InstructionExprAST(const std::string& name,
		std::vector<std::unique_ptr<ExprAST>> args)
		: _name(name), _args(std::move(args)) {}
};

#endif