#ifndef PARSER_HPP
#define PARSER_HPP

#include "lexer.hpp"
#include "AST.hpp"
#include <iostream>

class Parser {
	Lexer lexer;

	Token token;

	void NextToken() { token = lexer.GetToken(); }

	std::unique_ptr<ExprAST> LogError(std::string info) {
		fprintf(stderr, "Parser Error: %s \n", info.c_str()); 
		return nullptr;
	}

	std::unique_ptr<ExprAST> ParseInstruction() {
		if (token.identifier == "add") {
			// TODO: Refactor to a more modular approach
			
			std::vector <std::unique_ptr<ExprAST>> args;
			NextToken();
			if (token.identifier != "(") return LogError("Expected opening parethesis, got \"" + token.identifier + "\"");
			NextToken();
			if (token.type != number) return LogError("Invalid argument type; expected number, got " + token.type);
			args.push_back(ParseNumber());
			NextToken();
			if (token.identifier != ",") return LogError("Expected comma, got " + token.identifier);
			NextToken();
			if (token.type != number) return LogError("Invalid argument type; expected number, got " + token.type);
			args.push_back(ParseNumber());
			NextToken();
			if (token.identifier != ")") return LogError("Expected closing parethesis, got " + token.identifier);

			auto result = InstructionExprAST("add", args);
			return std::make_unique<InstructionExprAST>(result);
		}
	}

	std::unique_ptr<ExprAST> ParseNumber() {
		return std::make_unique<NumberExprAST>(token.value);
	}

public:
	Parser(Lexer& l) : lexer(l) {}

	inline std::unique_ptr<ExprAST> buildAST() {
		NextToken();

		switch (token.type) {
		case (eof):
			return nullptr;
		case (unknown):
			return LogError("Unknown type");
		case (instruction):
			return ParseInstruction();
		case (number):
			return ParseNumber();
		}
	}
};

#endif
