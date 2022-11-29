# FRAME
The FRAME programming language, built with LLVM

## Syntax
#### Basic structure
`instruction(parameter, parameter)`

The basic syntax of FRAME is a system where a built-in instruction takes in parameters which are surrounded by parenthesis and separated with a comma.

Modules for importing code


Structures for clumping variables and functions

Example function for calculating fibonacci numbers

    fn(fib, [x: int], int) {
        if smaller (x, 3)
            return (1)
        else
            return (add (do (fib, subtract (x, 1)), do (fib, subtract (x, 2)))
    }

### Instructions
See [instructions.md](instructions.md)
