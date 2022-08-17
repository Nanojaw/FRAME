# Development process 
This file contains a summary of the development process of FRAME.

### Contents:
- [Idea](#the-idea)
- [Syntax](#the-syntax)
- [LLVM](#using-llvm)

## The idea
Every project starts as an idea. This one was born from XPulseIV wanting to make his own language, and TuffenDuffen wanting to build an editor. However, with zero experience in any of the fields, things had to start small. The idea of a simple language with an accompanying editor was the result of those dreams.

Since both of us prefer compiled languages to interpreted ones, we had to design the language with making a compiler for it in mind. Therefore the language had to be very low level, since more abstraction meant more work for us. So, a compiled very low level language? Sounds like Assembly, which we have taken inspiration from. The WebAssembly text representation, WAT, has also played a big role in the design. 

We decided to make the language instruction based, just like x86 assembly or WAT, so that every line starts with a built-in instruction, making the compiler simpler to make. We also decided to strip away some features like registry access and type safety.

Fortran got to play a role with its modules and subroutines.

Now that we had a basic idea of the language, it was time for the next part.

## The Syntax

## Using LLVM
