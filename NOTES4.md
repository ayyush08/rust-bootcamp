# Macros
- Code that writes other code, expands to more rust code and provides a way to reduce boilerplate code.

-----------------------------------------------------------------------

## ```'println!'``` Macro
- Used to print text to the console.
- Without the '!' it would be a function call, but with the '!' it is a macro call meaning it doesn't  get called at runtime but rather at compile time gets expanded to more rust code.
- We can see this by running ```cargo expand``` in the terminal.
-  This will show us the expanded code that the macro generates.

- When you don't know the input size or type at compile time, macros are useful. 

- println! is a declarative macro, meaning it is defined using a pattern matching syntax.

- **When we compile a rust code containing a macro, a binary is created. The binary is a file that can be executed on the computer. But in between the rust code and the binary, there is a step called macro expansion. During this step, all the macros in the code are expanded to their corresponding code. This means that the macro call is replaced with the code that the macro generates.**

