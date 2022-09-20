#ifndef LEXER_HPP
#define LEXER_HPP

#include <string>
#include <iostream>

enum token_type {
    eof,
    unknown,
    instruction,
    number,
};

class Token {
public:
    token_type type;
    std::string identifier;
    int value;
};

class Lexer {
    std::istream& stream;
public:
    Lexer(std::istream& is) : stream(is) {}
    inline Token GetToken() {
        Token result = Token();

        int lastchar = ' ';

        while (isspace(lastchar)) {
            lastchar = stream.get();
        }

        if (isalpha(lastchar)) {
            result.type = instruction;
            result.identifier = lastchar;

            while (isalnum(lastchar = stream.get())) {
                result.identifier += lastchar;
            }

            return result;
        }

        if (isdigit(lastchar)) {
            result.type = number;

            
            std::string numStr;

            while (isdigit(lastchar)) {
                numStr += lastchar;
                lastchar = stream.get();
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
        return result;
    }
};

#endif