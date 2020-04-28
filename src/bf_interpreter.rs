use crate::token::*;
use std::collections::HashMap;
use std::collections::VecDeque;

/// Brainf*ck interpreter
pub struct BfInterpreter {
    token_array: Vec<Token>,
    token_ptr: u32,
    memory: Vec<u32>,
    memory_ptr: u16,
    input: VecDeque<char>,
    output: String,
    loop_start_end_token_ptr_map: HashMap<u32, u32>,
    loop_end_start_token_ptr_map: HashMap<u32, u32>,
}
impl BfInterpreter {

    /// creates new `BfInterpreter`
    /// 
    /// # Arguments
    /// 
    /// * `src` - source code to execute
    /// * `input` - input
    /// # Returns
    /// 
    /// `BfInterpreter`
    /// 
    pub fn new(src: &str, input: &str) -> BfInterpreter {
        let ta = Token::tokenize_from_array(src.chars().collect::<Vec<char>>());
        let (lsetpm, lestpm) = Token::get_loop_token_ptr(&ta);

        BfInterpreter {
            token_array: ta,
            token_ptr: 0,
            //Brainf*ck's number of memory cell is defined to be larger than 30,000.
            //So this program should reserve size of `u16::max_value()`,
            //which is expected to be 2^16 - 1 = 65,535.
            memory: vec![0; u16::max_value() as usize],
            memory_ptr: 0,
            input: input.chars().collect(),
            output: String::from(""),
            loop_start_end_token_ptr_map: lsetpm,
            loop_end_start_token_ptr_map: lestpm,
        }
    }

    /// executes the source code
    pub fn exec(&mut self) {
        let token_array = &self.token_array;
        while let Some(token) = token_array.get(self.token_ptr as usize) {
            match *token {
                Token::Inc => {
                    Token::inc_mem_val(&mut self.memory, self.memory_ptr);
                }
                Token::Dec => {
                    Token::dec_mem_val(&mut self.memory, self.memory_ptr);
                }
                Token::IncPtr => {
                    Token::inc_mem_ptr(&mut self.memory_ptr);
                }
                Token::DecPtr => {
                    Token::dec_mem_ptr(&mut self.memory_ptr);
                }
                Token::StartLoop => {
                    Token::jump_loop_end_token_if_mem_0(
                        self.memory.get(self.memory_ptr as usize),
                        &self.loop_start_end_token_ptr_map,
                        &mut self.token_ptr,
                    );
                }
                Token::EndLoop => {
                    Token::jump_loop_start_token_if_mem_not_0(
                        self.memory.get(self.memory_ptr as usize),
                        &self.loop_end_start_token_ptr_map,
                        &mut self.token_ptr,
                    );
                }
                Token::Input => {
                    Token::put_char_from_input_to_mem(
                        &mut self.input,
                        &mut self.memory,
                        self.memory_ptr,
                    );
                }

                Token::Output => {
                    Token::join_output_char_to_str(
                        self.memory
                            .get(self.memory_ptr as usize)
                            .and_then(|i| std::char::from_u32(*i)),
                        &mut self.output,
                    );
                }
            }
            self.token_ptr += 1;
        }
    }

    /// returns result
    pub fn output(&self) -> &str {
        return &self.output;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn hello_world() {
        let src = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.------------.<++++++++.--------.+++.------.--------.>+.";
        let input = "";
        let mut bf = BfInterpreter::new(src, input);
        bf.exec();
        assert_eq!(bf.output(), "Hello, world!");
    }
}
