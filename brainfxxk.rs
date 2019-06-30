use std::collections::VecDeque;
struct BfInterpreter{
    token_array : Vec<char>,
    token_ptr: u32,
    memory : Vec<u32>,
    memory_ptr: u16,
    input : VecDeque<char>,
    output : String,
}

enum TokenType {
    Inc,
    Dec,
    IncPtr,
    DecPtr,
    StartLoop,
    EndLoop,
    Input,
    Output,
    None,
}

impl TokenType{
    fn from_char(c:char) -> TokenType{
        match c{
            '+' => TokenType::Inc,
            '-' => TokenType::Dec,
            '>' => TokenType::IncPtr,
            '<' => TokenType::DecPtr,
            '[' => TokenType::StartLoop,
            ']' => TokenType::EndLoop,
            ',' => TokenType::Input,
            '.' => TokenType::Output,
             _  => TokenType::None,
        }
    }
}

impl BfInterpreter{
    fn init(src: &str, input: &str) -> BfInterpreter {
        BfInterpreter{
            token_array : src.chars().collect(),
            token_ptr : 0,
            //Brainf*ck's number of memory cell is defined to be larger than 30,000.
            //So this program should reserve size of u16::max_value(), 
            //which is expected to be 2^16 = 65,536.
            memory : vec![0 ; u16::max_value() as usize],
            memory_ptr : 0,
            input : input.chars().collect(),
            output : String::from(""),
        }
    }

    fn exec(&mut self){
        while let Some(token) = self.token_array.get(self.token_ptr as usize).clone(){
            match TokenType::from_char(*token){
                TokenType::Inc => {
                    BfInterpreter::inc_mem_val(&mut self.memory, self.memory_ptr);
                }
                TokenType::Dec => {
                    BfInterpreter::dec_mem_val(&mut self.memory, self.memory_ptr);
                }
                TokenType::IncPtr => {
                    BfInterpreter::inc_mem_ptr(&mut self.memory_ptr);
                }
                TokenType::DecPtr => {
                    BfInterpreter::dec_mem_ptr(&mut self.memory_ptr);
                }
                TokenType::StartLoop => {
                    BfInterpreter::jump_loop_end_token_if_mem_0(
                        self.memory.get(self.memory_ptr as usize),
                        &self.token_array, &mut self.token_ptr
                        );
                }
                TokenType::EndLoop => {
                    BfInterpreter::jump_loop_start_token_if_mem_not_0(
                        self.memory.get(self.memory_ptr as usize),
                        &self.token_array, &mut self.token_ptr
                        );
                }
                TokenType::Input => {
                    BfInterpreter::put_char_from_input_to_mem(
                        &mut self.input, 
                        &mut self.memory, self.memory_ptr
                        );
                }

                TokenType::Output => {
                    BfInterpreter::join_output_char_to_str(
                        self.memory.get(self.memory_ptr as usize)
                        .and_then(|i| std::char::from_u32(*i)),
                        &mut self.output);
                }
                TokenType::None => {}
            }
            self.token_ptr += 1;
        }
    }
    //fn for token '+'.
    fn inc_mem_val(memory :&mut Vec<u32>, memory_ptr:u16){
        if let Some(val) = memory.get_mut(memory_ptr as usize) {
            *val += 1;
        }
    }
    //fn for token '-'.
    fn dec_mem_val(memory :&mut Vec<u32>, memory_ptr:u16){
        if let Some(val) = memory.get_mut(memory_ptr as usize) {
            *val -= 1;
        }
    }
    //fn for token '>'.
    fn inc_mem_ptr(memory_ptr:&mut u16){
        *memory_ptr +=1;
    }
    //fn for token '<'.
    fn dec_mem_ptr(memory_ptr:&mut u16){
        *memory_ptr -=1;
    }
    //fn for token '['.
    fn jump_loop_end_token_if_mem_0(mem_val:Option<&u32>, 
                                        token_array:&Vec<char>, 
                                        token_ptr : &mut u32){
        if let Some(val) = mem_val{
            if *val != 0{return;}
        }else{return;}
            
        let mut cnt : u32 = 0;
        let mut ptr : u32 = *token_ptr + 1;
        while let Some(token) = token_array.get(ptr as usize).clone(){
            match TokenType::from_char(*token){
                TokenType::StartLoop => {cnt += 1;}
                TokenType::EndLoop => {
                    if cnt == 0 {
                        *token_ptr = ptr;
                        break;
                    }else{
                        cnt -= 1;
                    }
                }
                _ => {}
            }
            ptr += 1;
        }
    }
    //fn for token ']'.
    fn jump_loop_start_token_if_mem_not_0(mem_val:Option<&u32>, 
                                        token_array:&Vec<char>, 
                                        token_ptr : &mut u32){
        if let Some(val) = mem_val{
            if *val == 0{return;}
        }else{return;}
            
        let mut cnt : u32 = 0;
        let mut ptr : u32 = *token_ptr - 1;
        while let Some(token) = token_array.get(ptr as usize).clone(){
            match TokenType::from_char(*token){
                TokenType::EndLoop => {cnt += 1;}
                TokenType::StartLoop => {
                    if cnt == 0 {
                        *token_ptr = ptr;
                        break;
                    }else{
                        cnt -= 1;
                    }
                }
                _ => {}
            }
            ptr -= 1;
        }
    }
    //fn for token ','.
    fn put_char_from_input_to_mem(input_char_array:&mut VecDeque<char>,
                                  memory :&mut Vec<u32>, memory_ptr:u16){
        if let Some(val) = memory.get_mut(memory_ptr as usize){
            if let Some(c) = input_char_array.pop_front() {
                *val = u32::from(c);
            }
        }

    }
    //fn for token '.'.
    fn join_output_char_to_str(output_char:Option<char>, output_str:&mut String){
        if let Some(c) = output_char{
            output_str.push(c);
        }
    }
}

fn main (){
    let src: &str = 
        "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.
        ------------.<++++++++.--------.+++.------.--------.>+.";
    let input : &str = "";
    let mut  bf = BfInterpreter::init(src, input);
    bf.exec();
    println!("{}",bf.output);
}
