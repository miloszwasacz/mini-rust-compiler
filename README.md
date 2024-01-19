# μRust compiler

A simple compiler to LLVM backend for a toy language with Rust-like syntax.

## Usage requirements

- LLVM 16
    - If you are using Windows, see [this gist](https://gist.github.com/miloszwasacz/94ccdf0c16941b2586e3663a4c647363)
    - If you are using Ubuntu or Debian, use the script provided [here](https://apt.llvm.org/)
    - Otherwise, follow the instructions [here](https://crates.io/crates/llvm-sys)

## Build requirements

- LLVM 16
    - [_See usage requirements_](#usage-requirements)
- [Rust 1.72.1 or newer](https://www.rust-lang.org/tools/install)
    - _Older versions may work, but they have not been tested_