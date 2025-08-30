# Jujutsu Playground

A tiny Rust playground crate used to experiment with modules, documentation comments, tests, and basic console output. The program prints a short sequence of greeting/farewell messages.

## Overview

This repository demonstrates a minimal Rust binary project with:

- A main entry point that prints messages
- Two small modules (farewell and goodbye)
- Rustdoc-style documentation comments
- A basic unit test

It is intended as a simple starting point for learning and experimentation.

## Project structure

- Cargo.toml — crate metadata and dependencies
- src/main.rs — program entry point; calls the module functions
- src/farewell.rs — exposes `farewell()` that prints "Farewell!!"
- src/goodbye.rs — exposes `goodbye(name: &str)` that prints "Good bye {name}!!!" and contains a trivial test

## Prerequisites

- Rust toolchain (rustc and cargo). Install via https://rustup.rs

## Build

- Debug build: `cargo build`
- Release build: `cargo build --release`

## Run

Run the binary from the project root:

```
cargo run
```

Expected example output:

```
Hello, world!
Farewell!!
Good bye Noa!!!
```

Note: The name passed to `goodbye` in `main` is currently hard-coded as "Noa" (see src/main.rs). You can change it to any string you like.

## Test

Run tests with:

```
cargo test
```

There is a basic test in `src/goodbye.rs` as a placeholder.

## Extending

- Add command-line argument parsing (e.g., with `clap`) to accept a custom name at runtime.
- Replace print statements with structured logging if needed.
- Add more examples and tests to explore Rust’s module system and documentation features.

## License

This project is licensed under the BSD 3-Clause License. See the LICENSE file for details.
