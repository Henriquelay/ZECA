# ZECA
## Zero Efficiency Compiler: Augmented
---
Rust -> LLVM compiler written in Rust using [LALRPOP] library.

# Requirements
* Rust 2021
* Cargo
> To install/update Rust, follow [this link][Rust install] for instructions on the official website.

# Using the compiler
## Without installation
While in any folder withing the project, run:
```sh
cargo run path/to/source/file # or alternatively
cargo r path/to/source/file
```

## Installing
While on project root folder, run:
```sh
cargo install --path .
```

This will add `zeca` to `$HOME/.cargo/bin` which should be in your `$PATH` if you installed through `rustup`. 
Then, simply call `zeca path/to/source/file`.

# Tests
Unit tests are avaiable under `src/test` as test modules for Cargo, as per [test crate][https://doc.rust-lang.org/stable/test/]'s specification.

## Running tests
While in any folder withing the project, run:
```sh
cargo test # or alternatively
cargo t
```

Ignored tests are tests that were once in the compiler's score but were simplified away. 
Integration tests are yet to be added.


[LALRPOP] https://github.com/lalrpop/lalrpop
[Rust install] https://www.rust-lang.org/tools/install
