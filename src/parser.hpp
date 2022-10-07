#pragma once

#include <map>

#include "AST.hpp"
#include "lexer.hpp"

class cParser
{
    cLexer _lexer;

    cToken _token;

    const std::map<std::string, int> _instruction_map = {
        {"comment", AST::comment},
        {"add", AST::add}
    };

    auto NextToken() -> void { _token = _lexer.GetToken(); }

    static auto LogError(const std::string& info) -> std::unique_ptr<AST::Expr>
    {
        fprintf(stderr, "Parser Error: %s \n", info.c_str());
        return nullptr;
    }

    auto ParseInstruction() -> std::unique_ptr<AST::Expr>
    {
        if (_token.Identifier == "add")
        {
            // TODO: Refactor to a more modular approach

            std::vector<std::unique_ptr<AST::Expr>> args;
            NextToken();
            if (_token.Identifier != "(") return LogError("Expected opening parenthesis, got \"" + _token.Identifier + "\"");
            NextToken();
            if (_token.Type != number) return LogError("Invalid argument type; expected number, got " + _token.Type);
            args.push_back(ParseNumber());
            NextToken();
            if (_token.Identifier != ",") return LogError("Expected comma, got " + _token.Identifier);
            NextToken();
            if (_token.Type != number) return LogError("Invalid argument type; expected number, got " + _token.Type);
            args.push_back(ParseNumber());
            NextToken();
            if (_token.Identifier != ")") return LogError("Expected closing parenthesis, got " + _token.Identifier);

            return std::make_unique<AST::InstructionExpr>(AST::add, std::move(args));
        }
        return {};
    }

    auto ParseNumber() -> std::unique_ptr<AST::Expr>
    {
        return std::make_unique<AST::NumberExpr>(_token.Value);
    }

    static auto ParseModule() -> std::unique_ptr<AST::Expr>
    {
        return nullptr;
    }

public:
    explicit cParser(const cLexer& l) : _lexer(l), _token{} {}

    auto BuildAst() -> std::unique_ptr<AST::Expr>
    {
        NextToken();

        switch (_token.Type)
        {
        case (eof):
            return nullptr;
        case (unknown):
            return LogError("Unknown type");
        case (instruction):
            return ParseInstruction();
        case (number):
            return ParseNumber();
        case (module):
            return ParseModule();
        }
        return nullptr;
    }
};
