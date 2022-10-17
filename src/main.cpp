#include <iostream>
#include <fstream>
#include <filesystem>

#include "lexer.hpp"
#include "parser.hpp"

int main()
{
	std::ifstream f("./src/test.frame", std::ifstream::in);
	if (f.good() != true) { 
		std::cout << "Could not find ../src/test.frame\n";
		std::cout << "Current path: " << std::filesystem::current_path() << std::endl;
		return 1;
	}
	const auto lexer = Lexer(f);
	auto parser = Parser(lexer);

	auto test = parser.buildAST();

	f.close();
	return 0;
}