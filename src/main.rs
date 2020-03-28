mod brainfxxk;

fn main (){
    let src: &str = 
        "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.
        ------------.<++++++++.--------.+++.------.--------.>+.";
    let input : &str = "";
    let mut  bf = brainfxxk::BfInterpreter::new(src, input);
    bf.exec();
    println!("{}",bf.output());
}
