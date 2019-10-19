mod brainfxxk;

fn main (){
    let src: &str = 
        "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.
        ------------.<++++++++.--------.+++.------.--------.>+.";
    let input : &str = "";
    let mut  bf = brainfxxk::BfInterpreter::init(src, input);
    bf.exec();
    println!("{}",bf.output());
}
