use brainfxxk::bf_interpreter::*;

fn main() {
    let src: &str = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.
        ------------.<++++++++.--------.+++.------.--------.>+.";
    let input: &str = "";
    let mut bf = BfInterpreter::new(src, input);
    bf.exec();
    println!("{}", bf.output());
}
