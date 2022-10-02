#pragma once

#include <map>

#include "lexer.hpp"
#include "AST.hpp"

class Parser {
	Lexer lexer;

	Token token;

	const std::map<std::string, int> instruction_map = {
		{"comment", AST::comment},
		{"add", AST::add}
	};

	void NextToken() { token = lexer.GetToken(); }

	static std::unique_ptr<AST::Expr> LogError(const std::string& info) {
		fprintf(stderr, "Parser Error: %s \n", info.c_str());
		return nullptr;
	}

	std::unique_ptr<AST::Expr> ParseInstruction() {
		if (token.identifier == "add") {
			// TODO: Refactor to a more modular approach

			std::vector <std::unique_ptr<AST::Expr>> args;
			NextToken();
			if (token.identifier != "(") return LogError("Expected opening parenthesis, got \"" + token.identifier + "\"");
			NextToken();
			if (token.type != number) return LogError("Invalid argument type; expected number, got " + token.type);
			args.push_back(ParseNumber());
			NextToken();
			if (token.identifier != ",") return LogError("Expected comma, got " + token.identifier);
			NextToken();
			if (token.type != number) return LogError("Invalid argument type; expected number, got " + token.type);
			args.push_back(ParseNumber());
			NextToken();
			if (token.identifier != ")") return LogError("Expected closing parenthesis, got " + token.identifier);

			return std::make_unique<AST::InstructionExpr>(AST::add, std::move(args));
		}
	}

	std::unique_ptr<AST::Expr> ParseNumber() {
		return std::make_unique<AST::NumberExpr>(token.value);
	}

	std::unique_ptr<AST::Expr> ParseModule() {
		return nullptr;
	}

public:
	Parser(const Lexer& l) : lexer(l) {}

	inline std::unique_ptr<AST::Expr> buildAST() {
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
		case (module):
			return ParseModule();
		}
		return nullptr;
	}
};
