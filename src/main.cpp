#include <fstream>

#include "modparser.hpp"

auto main() -> int
{
    std::ifstream f("../src/test.frame", std::ifstream::in);
    if (f.good() != true) return 1;
    auto parser = modparser::cParser(f);

    //parser

    f.close();
    return 0;
}