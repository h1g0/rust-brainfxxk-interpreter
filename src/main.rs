
use anyhow::Result;
use brainfxxk::bf_interpreter::*;

fn main()->Result<()>{
    let src = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.------------.<++++++++.--------.+++.------.--------.>+.";
    let input = "";
    let mut bf = BfInterpreter::new(src, input)?;
    bf.exec()?;
    println!("{}", bf.output());
    Ok(())
}
