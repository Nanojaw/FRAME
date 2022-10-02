#include <iostream>
#include <fstream>
#include <filesystem>

#include "lexer.hpp"
#include "parser.hpp"
#include "modparser.hpp"

int main()
{
	std::ifstream f("../src/test.frame", std::ifstream::in);
	if (f.good() != true) return 1;
	const auto lexer = Lexer(f);
	auto parser = Parser(lexer);

	auto test = parser.buildAST();

	f.close();
	return 0;
}