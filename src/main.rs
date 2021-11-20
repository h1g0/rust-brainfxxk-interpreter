use brainfxxk::bf_interpreter::*;

fn main() {
    let src = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.------------.<++++++++.--------.+++.------.--------.>+.";
    let input = "";
    let mut bf = BfInterpreter::new(src, input).unwrap();
    bf.exec().unwrap();
    println!("{}", bf.output());
}
