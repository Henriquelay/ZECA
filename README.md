# ZECA

Zero Efficiency Compiler: Augmented

A Rust -> LLVM compiler written in Rust using [Chumsky] library.

## Requirements

* Rust 2021
* Cargo
* *Optional*: Rustdoc

> To install/update Rust, follow [this link][Rust install] for instructions on the official website.

## Using the compiler

It is possible to build the project in several ways:
### Without installation

While in any folder within the project, run:

```sh
cargo run --release path/to/source/file # or, alternatively:
cargo r --release path/to/source/file
```

Or, to run in *debug mode*:

```sh
cargo run path/to/source/file # or, alternatively:
cargo r path/to/source/file
```

### Installing

While on the project root folder, run:

```sh
cargo install --path .
```

This will build with release mode and add `zeca` to `$HOME/.cargo/bin` which should be in your `$PATH` if you installed Rust through `rustup`.
Then, simply call `zeca path/to/source/file`.

## Tests

Unit tests are available under `src/` as test modules for Cargo.
Integration tests are available under `tests/` and may either read from example files (`tests/examples`) or input directly as hardcoded strings, when arbitrarily deemed simple enough.

Both are discarded in the final binary (virtue of using `--release` flag).

### Running tests

While in any folder withing the project, run:

```sh
cargo test # or, alternatively
cargo t
```

>Ignored tests are tests that were once in the compiler's scope but were simplified away due to the project's due date and the discipline's scope, or some other reason.

## Documentation

To build the documentation website for ZECA and Chumsky (the major library used on this project) using Rustdoc's Cargo integration, if you have Rustdoc installed (included by default if installed with Rustup's toolchain), simply run:

```sh
cargo doc --package zeca --package chumsky --no-deps --release
```

You can then conveniently read the docs by running

```sh
cargo doc --open
```

*Other dependencies are hidden to avoid confusion.

## License

ZECA is licensed under the GPL-3 license (for reference, see the `LICENSE` file), which is compatible with [Chumsky]'s MIT license.

Since ZECA builds heavily upon Chumsky, we'd like to thank [Chumsky] maintainers!

[Chumsky]: https://github.com/zesterer/chumsky
[Rust install]: https://www.rust-lang.org/tools/install
