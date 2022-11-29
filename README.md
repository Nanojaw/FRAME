# FRAME
The FRAME programming language, built with LLVM

## Syntax
#### Basic structure
`instruction(parameter, parameter)`

The basic structure of FRAME is built on a system where a built-in instructionhe parameters are surrounded by parenthesis and separated with a comma.


modules for importing code

structures for clumping variables and functions

block = pointer with size

    fn(fib, [x: int], int) {
        if smaller (x, 3)
            return (1)
        else
            return (add (do (fib, subtract (x, 1)), do (fib, subtract (x, 2)))
    }

### Instructions
See [instructions.md](instructions.md)
