#ifndef LEXER_HPP
#define LEXER_HPP

#include <string>

enum token_type {
    eof = -1,
    instruction = -2,
    number = -3
};

class Token {
public:
    token_type type;
    std::string identifier;
    int value;
};

class Lexer {
public:
    inline Token getToken() {
        Token result = Token();

        int lastchar = ' ';

        while (isspace(lastchar)) {
            lastchar = getchar();
        }

        if (isalpha(lastchar)) {
            result.identifier = lastchar;

            while (isalnum(lastchar = getchar())) {
                result.identifier += lastchar;
            }

            return result;
        }

        if (isdigit)
    }
};

#endif