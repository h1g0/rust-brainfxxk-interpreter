# rust-brainfxxk-interpreter

![Build on Ubuntu](https://github.com/h1g0/rust-brainfxxk-interpreter/workflows/Build%20on%20Ubuntu/badge.svg)
![Build on macOS](https://github.com/h1g0/rust-brainfxxk-interpreter/workflows/Build%20on%20macOS/badge.svg)
![Build on Windows](https://github.com/h1g0/rust-brainfxxk-interpreter/workflows/Build%20on%20Windows/badge.svg)

[Brainf*ck](https://en.wikipedia.org/wiki/Brainfuck) interpreter written in Rust.

## Usage

see the example code below:

```rust
use brainfxxk::bf_interpreter::*;
let src = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.------------.<++++++++.--------.+++.------.--------.>+.";
let input = "";
let mut bf = BfInterpreter::new(src, input).unwrap();
bf.exec().unwrap();
assert_eq!(bf.output(), "Hello, world!");
```
