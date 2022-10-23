#include "modparser.hpp"

#include <filesystem>
#include <fstream>
#include <iostream>

auto main() -> int {
  std::ifstream f("./src/test.frame", std::ifstream::in);
  if (f.good() != true) {
    std::cout << "Could not find ../src/test.frame\n";
    std::cout << "Current path: " << std::filesystem::current_path() << std::endl;
    return 1;
  }
  auto parser = modparser::cParser(f);
  //parser

  f.close();
  return 0;
}