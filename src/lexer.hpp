#pragma once

#include <string>
#include <istream>

enum token_type {
    eof,
    unknown,
    instruction,
    number,
    module,
};

class Token {
public:
    token_type type;
    std::string identifier;
    int value;
};

class Lexer {
    std::istream& stream;
    int lastchar = ' ';
public:
    Lexer(std::istream& is) : stream(is) {}
    inline Token GetToken() {
        Token result = Token();

        while (isspace(lastchar)) {
            lastchar = stream.get();
        }

        if (isalpha(lastchar)) {
            result.identifier = lastchar;

            while (isalnum(lastchar = stream.get())) {
                result.identifier += lastchar;
            }

            if (result.identifier == "mod") {
                result.type = module;
                return result;
            }

            result.type = instruction;
            return result;
        }

        if (isdigit(lastchar)) {
            result.type = number;


            std::string numStr;
            numStr = lastchar;

            while (isdigit(lastchar = stream.get())) {
                numStr += lastchar;
            }

            result.value = std::stoi(numStr);
            return result;
        }

        if (lastchar == EOF) {
            result.type = eof;
            return result;
        }

        result.type = unknown;
        result.identifier += lastchar;
        lastchar = stream.get();
        return result;
    }
};