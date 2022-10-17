// The modparser is the component which reads and parses a file into an AST
// which the other components can use. It consists of a lexer, which reads the
// input and turns it into tokens, and the parser, which turns the tokens into
// an AST. Since the lexer should not be used elsewhere than the parser, the
// parser creates the lexer to ensure that nothing is skipped.
#pragma once

#include <istream>
#include <map>
#include <string>
#include "AST.hpp"

namespace modparser
{
    enum lex_token_type
    {
        eof,
        unknown,
        instruction,
        number,
        module,
    };

    class cLexToken
    {
    public:
        lex_token_type Type;
        std::string Identifier;
        int Value;
    };

    class cLexer
    {
        std::istream& _input_stream_;

    public:
        explicit cLexer(std::istream& is) : _input_stream_(is) {}
    };

    class cParser
    {
        cLexer _lexer_;

        cLexToken _token;

        const std::map<std::string, int> _instruction_map = {
            {"comment", AST::comment},
            {"add", AST::add}
        };

        //void NextToken() { token = lexer_.GetToken(); }

    public:
        explicit cParser(std::istream& is) : _lexer_(cLexer(is)), _token{} {}
    };
}