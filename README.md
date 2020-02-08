# py2rust

`py2rust` is a transpiler that converts `Python` to `Rust`.

## Documents

Design

https://github.com/rollrat/py2rust/blob/master/doc/design.md

Development

https://github.com/rollrat/py2rust/blob/master/doc/dev.md

## Why do we make this?

`Python` is a very productive language, and `Rust` is a very stable language.

Many high-level projects are implemented in Python. (youtube-dl, tensorflow, …)

Rust can be used in many development areas. (System, WebAssembly, Android, …)

Using Python with Rust together allows for high productivity, easy maintenance, stable static linking, LLVM optimizations, or AOT compilation features.

## Future

Finding `memory leaks` is a very difficult problem.
`Rust` allows you to manage memory safely and optimally.
You can use `Rust` model to convert existing code to code that does not have a `memory leak`.
The success of this project is no different than finding a way to solve `memory leaks` in existing languages.
So by making this project successful, users will know how to get code stability.

However, languages that support pointers will be difficult to apply because of the enormous complexity of alias analysis.
Therefore, I will only consider high-level languages.