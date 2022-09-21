#include <iostream>
#include <fstream>
#include "lexer.hpp"
#include "parser.hpp"

int main()
{
	std::ifstream f("test.frame", std::ifstream::in);
	if (f.good() != true) return 1;
	auto lexer = Lexer(f);
	auto parser = Parser(lexer);

	auto test = parser.buildAST();

	f.close();
	return 0;
}