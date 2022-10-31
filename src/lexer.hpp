#pragma once

#include <istream>
#include <map>
#include <memory>
#include <string>

namespace Lexer {
  enum LexType { eof, unknown, module, instruction, number };

  enum InstructionType { add };

  enum NumberType {
    natural, // any positive whole number
    integer, // any whole number
    rational // any number that is writable with digits
  };

  std::map<std::string, InstructionType> InstructionFromStrMap = {
    {"add", add}
  };

  class LexToken {
  public:
    const LexType Type;

    LexToken(LexType type) : Type(type) { }
  };

  class UnknownToken : public LexToken {
  public:
    const std::string Identifier;

    UnknownToken(const std::string& identifier) : Identifier(identifier), LexToken(unknown) { }
  };

  class ModToken : public LexToken {
  public:
    const std::string Name;

    ModToken(const std::string& name) : Name(name), LexToken(module) { }
  };

  class InstructionToken : public LexToken {
  public:
    const InstructionType Instruction;

    InstructionToken(const InstructionType instructionType) : Instruction(instructionType), LexToken(instruction) { }
  };

  template<typename T> class NumberToken : public LexToken {
  public:
    const NumberType Number;
    const T Value;

    NumberToken(const NumberType numberType, T value) : Number(numberType), Value(value), LexToken(number) { }
  };

  class Lexer {
    std::istream& m_InputStream;
    int m_LastChar;

  public:
    explicit Lexer(std::istream& is) : m_InputStream(is), m_LastChar(m_InputStream.get()) { }

    auto GetNextToken() -> std::unique_ptr<LexToken> {
      while (isspace(m_LastChar)) m_LastChar = m_InputStream.get();

      if (isalpha(m_LastChar)) {
        std::string identifier = std::to_string(m_LastChar);

        while (isalnum(m_LastChar = m_InputStream.get())) identifier += std::to_string(m_LastChar);

        if (identifier == "mod") {
          std::string name = "";

          while (isalnum(m_LastChar = m_InputStream.get())) name += std::to_string(m_LastChar);

          return std::make_unique<ModToken>(name);
        }

        if (InstructionFromStrMap.contains(identifier)) {
          return std::make_unique<InstructionToken>(InstructionFromStrMap.at(identifier));
        }

        return std::make_unique<UnknownToken>(identifier);
      }

      if (m_LastChar == '-' || isdigit(m_LastChar)) {
        std::string numStr = std::to_string(m_LastChar);

        while (isdigit(m_LastChar = m_InputStream.get()) || m_LastChar == '.') {
          numStr += std::to_string(m_LastChar);
        }

        if (numStr.find('.')) {
          double numValue = std::stod(numStr);
          return std::make_unique<NumberToken<double>>(rational, numValue);
        } else if (numStr[0] == '-') {
          return std::make_unique<NumberToken<long long>>(integer, std::stoll(numStr));
        } else {
          return std::make_unique<NumberToken<unsigned long long>>(integer, std::stoull(numStr));
        }
      }

      if (m_LastChar == EOF) {
        return std::make_unique<LexToken>(eof);
      }

      return std::make_unique<UnknownToken>(std::to_string(m_LastChar));
    }
  };
} // namespace Lexer