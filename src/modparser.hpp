// The modparser is the component which reads and parses a file into an AST
// which the other components can use. It constists of a lexer, which reads the
// input and turns it into tokens, and the parser, which turns the tokens into
// an AST. Since the lexer should not be used elsewhere than the parser, the 
// parser creates the lexer to ensure that nothing is skipped.
#pragma once

#include <string>
#include <istream>

namespace modparser {
	enum lex_token_type {
		eof,
		unknown,
		instruction,
		number,
		module,
	};

	class lex_token {
	public:
		lex_token_type type;
		std::string identifier;
		int value;
	};

	class Lexer {
		std::istream& input_stream;
	public:
		Lexer(std::istream& is) : input_stream(is) {}
	};
	class parser {
		Lexer lexer;
	public:
		inline parser(std::istream& is) {
			lexer = Lexer(is);
		}
	};
}