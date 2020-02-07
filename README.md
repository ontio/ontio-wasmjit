# ontio-wasmjit


**ontio-wasmjit** is a high performance WebAssembly execution engine built for ontology, originally forked from [wasmtime](https://github.com/bytecodealliance/wasmtime)


## Features

- **High Performance** - Order of magnitude faster than interpreter execution, thanks to [cranelift](https://github.com/bytecodealliance/cranelift).
- **Deterministic** - Diasable all floating point operation, limit the function call depth and the number of function local variables.
- **Secure** - contains gas mentering, limit all resources usage.

