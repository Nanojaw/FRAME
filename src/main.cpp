#include <iostream>
#include <fstream>
#include <filesystem>

#include "modparser.hpp"

int main()
{
	std::ifstream f("../src/test.frame", std::ifstream::in);
	if (f.good() != true) return 1;
	modparser::parser parser = modparser::parser(f);

	parser.

	f.close();
	return 0;
}