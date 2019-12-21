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