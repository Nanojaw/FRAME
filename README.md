# FRAME
The FRAME programming language

## Syntax
#### Basic structure
`instruction place; parameter, parameter`

built-in instruction
where to save the result
parameters for the instruction, separated by a comma

modules for importing code

structures for clumping variables and functions

block = pointer with size

### Instructions
`do` calls a function : `do var; function_name, parameter`

`call` calls a subroutine : `call subroutine_name, parameter`

`use` imports a module : `use module_name`

`end` ends the current scope :

    fn name in parameter: size out result: size
        (some code)
    end

`for` loops a set of code a specific amount of times : 

    for i: size; start, end, increase
        (some code)
    end

`while` loops until a condition is met : 

`var` creates a new variable : `var name: size; value`
