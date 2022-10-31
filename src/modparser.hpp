// The modparser is the component which reads and parses a file into an AST
// which the other components can use. It consists of a lexer, which reads the
// input and turns it into tokens, and the parser, which turns the tokens into
// an AST. Since the lexer should not be used elsewhere than the parser, the
// parser creates the lexer to ensure that nothing is skipped.
#pragma once

#include "AST.hpp"
#include "lexer.hpp"

#include <istream>
#include <map>
#include <string>

namespace Modparser {
  class Parser {
    Lexer::Lexer m_Lexer;
    std::unique_ptr<Lexer::LexToken> m_Token;

    static auto LogError(const std::string& info) -> std::unique_ptr<AST::Expr>
    {
        fprintf(stderr, "Parser Error: %s \n", info.c_str());
        return nullptr;
    }

    auto ParseInstruction() -> std::unique_ptr<AST::Expr> {
      if(static_cast<Lexer::InstructionToken>(*m_Token)->)
    }

    auto ParseAdd() -> std::unique_ptr<AST::Expr> {
      std::vector<std::unique_ptr<AST::Expr>> inst_params;
      m_Token = m_Lexer.GetNextToken();

      if (m_Token.Identifier != "(") return LogError("Expected opening parenthesis, got \"" + m_Token.Identifier + "\"");

      m_Token = m_Lexer.GetNextToken();

      while (m_Token.Identifier != ")") {
        if (m_Token.Type == LexTokenType::instruction) {
          ParseInstruction();
        }

        if (m_Token.Identifier == ",") {
          continue;
        }

        if (m_Token.Type != number) return LogError("Invalid argument type; expected number, got " + m_Token.Type);
            inst_params.push_back(ParseNumber());
      }

    }

  public:
    explicit Parser(std::istream& is) : m_Lexer(Lexer::Lexer(is)), m_Token() { }

    // TODO: remake so it parses entire module
    auto ParseModule() -> std::unique_ptr<AST::Expr> {
      if (m_Token->Type == Lexer::LexType::instruction) return ParseInstruction();
    }
  };
} // namespace Modparser