# rust-brainfxxk-interpreter

[Brainf*ck](https://en.wikipedia.org/wiki/Brainfuck) interpreter written in Rust.

## Usage

see the example code below:

```rust
use brainfxxk::bf_interpreter::*;

fn hello_world() {
    let src = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.------------.<++++++++.--------.+++.------.--------.>+.";
    let input = "";
    let mut bf = BfInterpreter::new(src, input);
    bf.exec();
    assert_eq!(bf.output(), "Hello, world!");
}
```