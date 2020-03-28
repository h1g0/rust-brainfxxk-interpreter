#[cfg(test)]
mod brainfxxk;

#[test]
fn hello_world() {
    let src: &str = 
        "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.
        ------------.<++++++++.--------.+++.------.--------.>+.";
    let input : &str = "";
    let mut  bf = brainfxxk::BfInterpreter::new(src, input);
    bf.exec();
    assert_eq!(bf.output(), "Hello, world!");
}
