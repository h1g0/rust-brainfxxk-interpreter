use brainfxxk::bf_interpreter::*;

fn main() {
    let src = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.------------.<++++++++.--------.+++.------.--------.>+.";
    let input = "";
    let mut bf = BfInterpreter::new(src, input);
    bf.exec();
    println!("{}", bf.output());
}
