# Development process 
This file contains a summary of the development process of FRAME.

### Contents:
- [Idea](#the-idea)
- [Syntax](#the-syntax)
- [LLVM](#using-llvm)

## The idea
Every project starts as an idea. This one was born from XPulseIV wanting to make his own language, and TuffenDuffen wanting to build an editor. However, with zero experience in any of the fields, things had to start small. The idea of a simple language with an accompanying editor was the result of those dreams.

Since both of us prefer compiled languages to interpreted ones, we had to design the language with making a compiler for it in mind. Therefore the language had to be very low level, since more abstraction meant more work for us. So, a compiled very low level language? Sounds like Assembly, which we have taken inspiration from. The WebAssembly text representation, WAT, has also played a big role in the design. 

We decided to make the language instruction based, just like x86 assembly, WASM WAT or LLVM IR, so that every line starts with a built-in instruction, making the compiler simpler to make. We also decided to strip away some features like registry access.

Fortran got to play a role with its modules and subroutines.

Now that we had a basic idea of the language, it was time for the next part.

## The Syntax
So, the design part! Out of all the languages I have listed, only one is meant to be written by humans, namely Fortran. However, Fortran is verbose, which we do not like, so we &borrowed some of the syntax from Rust, which is shorter and more pleasant to write. So, for example, instead of `module` and `function`, we get `mod` and `fn`.

Now, the structure of the instructions, which are the most fundamental part of our language. This structure has changed over time, as we ran into challenges we hadn't seen when deciding on the structure. The first iteration looked like: `instruction parameter, parameter`. Now, this was taken directly from x86 assembly, and isn't the best way to do things. The second iteration brought shorthand variables with a spot for where to save the value, making the syntax look like this: `instruction place; parameter, parameter`. This was pretty good until we added the option to put an instruction in the parameter slot for an instruction, because now we had nested instructions, but no way to differentiate where a instruction ended. Therefore we removed the semicolon and added parenthesis around the parameters. The new structure was `instruction place (parameter, parameter)`.

Instructions that don't return anything never had the `place`, since it was irrelevant, and when nesting came into the picture, nested instuctions also omitted the `place` since it was impiled. 

Now, all we had to do was define the syntax for some of the basic instructions, and start to build the language.

## Using LLVM
