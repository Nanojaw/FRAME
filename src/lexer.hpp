#pragma once

#include <istream>
#include <string>

enum token_type
{
    eof,
    unknown,
    instruction,
    number,
    module,
};

class cToken
{
public:
    token_type Type;
    std::string Identifier;
    int Value{};
};

class cLexer
{
    std::istream& _stream;
    int _lastchar = ' ';

public:
    explicit cLexer(std::istream& is) : _stream(is) {}

    auto GetToken() -> cToken
    {
        auto result = cToken();

        while (isspace(_lastchar))
            _lastchar = _stream.get();

        if (isalpha(_lastchar))
        {
            result.Identifier = std::to_string(_lastchar);

            while (isalnum(_lastchar = _stream.get()))
                result.Identifier += std::to_string(_lastchar);

            if (result.Identifier == "mod")
            {
                result.Type = module;
                return result;
            }

            result.Type = instruction;
            return result;
        }

        if (isdigit(_lastchar))
        {
            result.Type = number;


            std::string numStr = std::to_string(_lastchar);

            while (isdigit(_lastchar = _stream.get()))
                numStr += std::to_string(_lastchar);

            result.Value = std::stoi(numStr);
            return result;
        }

        if (_lastchar == EOF)
        {
            result.Type = eof;
            return result;
        }

        result.Type = unknown;
        result.Identifier += std::to_string(_lastchar);
        _lastchar = _stream.get();
        return result;
    }
};