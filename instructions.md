# List of instructions for FRAME

`set` sets or creates a variable. Usage: `set(var, value)` where var is the name of the variable and value is the value. Usable within Main and bodies.

# Arithmetic
`add` adds two or more values. Usable in parameters and structures. Usage `add(term, term...)`.

`sub` subtracts two or more values. Usable in parameters and structures. Usage `sub(term, term...)`. 

`mul` multiplies two or more value. Usable in parameters and structures. Usage `mul(factor, factor...)`.

`div` divides two or more values. Usable in parameters and structures. Usage `div(dividend, divisor...)` where extra values are treated as divisors, which means: $$  { dividend \over divisor } \over extra $$

`pow` powers a value by another. Usable in parameters and structures. Usage `pow(base, exponent...)` where extra values act as power to the power.

`rot` takes the root of a value. Usable in parameters and structures. Usage `rot(radicand, degree)`, and no additional values