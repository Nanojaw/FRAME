# FRAME
The FRAME programming language

## Syntax
#### Basic structure
`instruction place; parameter, parameter`

The basic structure of FRAME is built on a system where a built-in instruction is entered, this is then followed by a place for saving the result and a semicolon marking the beginning of the subsequent parameters. The parameters are separated by a comma.

In FRAME some instructions do not return anything and are therefore written `instruction parameter, parameter`

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

    while condition
        (some code)
    end

`var` creates a new variable : `var name: size; value`

`if` creates a conditional code block : 

    if condition
        (some code)
    end
    
`else if` creates a conditional code block which is evalutated if the previous if isn't run :

    if condition
        (some code)
    else if condition
        (some code)
    else if condition
        (some code)
    end
        
`else` creates a code block which is run if the previous if isn't :

    if condition
        (some code)
    else if condition
        (some code)
    else
        (some code)
    end
    
Note that else and else if end the scope of the previous if, and therefore `end` is only needed after the last `else`/`else if`
