#pragma once

#include <string>
#include <memory>
#include <vector>

#include "llvm/IR/Constants.h"

namespace AST {
enum instructions
{
    comment,
    add
};

class Expr {
public:
    virtual ~Expr() {}
    virtual llvm::Value* codegen() = 0;
};

class NumberExpr : public Expr {
    int _val;

public:
    NumberExpr(int val) : _val(val) {}
    llvm::Value* codegen() override;
};

class InstructionExpr : public Expr {
    int _instruction;
    std::vector<std::unique_ptr<Expr>> _args;
public:
    InstructionExpr(const int instruction,
        std::vector<std::unique_ptr<Expr>> args)
        : _instruction(instruction), _args(std::move(args)) {}
    llvm::Value* codegen() override;
};
}