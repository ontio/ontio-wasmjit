# ontio-wasmjit

**ontio-wasmjit** is a high performance WebAssembly execution engine built for ontology.

## Features

- **High Performance** - Order of magnitude faster than interpreter execution, thanks to [cranelift](https://github.com/bytecodealliance/cranelift).
- **Deterministic** - Diasable all floating point operation, limit the function call depth and the number of function local variables.
- **Secure** - contains gas mentering, limit all resources usage.


## License

This project is licensed under the [MIT license](LICENSE).

### Third party software

This project is originally forked from [wasmtime](https://github.com/bytecodealliance/wasmtime), which licensed under the Apache License (Version 2.0).
