// The modparser is the component which reads and parses a file into an AST
// which the other components can use. It consists of a lexer, which reads the
// input and turns it into tokens, and the parser, which turns the tokens into
// an AST. Since the lexer should not be used elsewhere than the parser, the
// parser creates the lexer to ensure that nothing is skipped.
#pragma once

#include "AST.hpp"

#include <istream>
#include <map>
#include <string>

namespace modparser {
  enum lex_token_type {
    eof,
    unknown,
    instruction,
    number,
    module,
  };

  class cLexToken {
  public:
    lex_token_type Type;
    std::string Identifier;
    int Value;
  };

  class cLexer {
    std::istream& _input_stream_;
    int lastchar;

  public:
    explicit cLexer(std::istream& is) : _input_stream_(is), lastchar(_input_stream_.get()) { }

    auto GetNextToken() -> cLexToken {
      auto result = cLexToken();

      while (isspace(lastchar)) lastchar = _input_stream_.get();

      if (isalpha(lastchar)) {
        result.Identifier = std::to_string(lastchar);

        while (isalnum(lastchar = _input_stream_.get())) result.Identifier += std::to_string(lastchar);

        if (result.Identifier == "mod") {
          result.Type = module;
          return result;
        }

        result.Type = instruction;
        return result;
      }

      if (isdigit(lastchar)) {
        result.Type = number;

        std::string numStr = std::to_string(lastchar);

        while (isdigit(lastchar = _input_stream_.get())) numStr += std::to_string(lastchar);

        result.Value = std::stoi(numStr);
        return result;
      }

      if (lastchar == EOF) {
        result.Type = eof;
        return result;
      }

      result.Type = unknown;
      result.Identifier += std::to_string(lastchar);
      lastchar = _input_stream_.get();
      return result;
    }
  };

  class cParser {
    cLexer _lexer_;

    cLexToken _token;

    const std::map<std::string, int> _instruction_map = {
      {"add", AST::add}
    };

    //void NextToken() { token = lexer_.GetToken(); }

  public:
    explicit cParser(std::istream& is) : _lexer_(cLexer(is)), _token{} { }
  };
} // namespace modparser