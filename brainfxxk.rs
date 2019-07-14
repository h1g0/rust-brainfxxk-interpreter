use std::collections::VecDeque;
use std::collections::HashMap;
enum TokenType {
    Inc,
    Dec,
    IncPtr,
    DecPtr,
    StartLoop,
    EndLoop,
    Input,
    Output,
}

impl TokenType{
    fn tokenize(c:char) -> Option<TokenType>{
        match c{
            '+' => Some(TokenType::Inc),
            '-' => Some(TokenType::Dec),
            '>' => Some(TokenType::IncPtr),
            '<' => Some(TokenType::DecPtr),
            '[' => Some(TokenType::StartLoop),
            ']' => Some(TokenType::EndLoop),
            ',' => Some(TokenType::Input),
            '.' => Some(TokenType::Output),
             _  => None,
        }
    }

    fn tokenize_from_array(char_array: Vec<char>)->Vec<TokenType>{
        let mut token_array: Vec<TokenType> = Vec::new();
        for c in char_array{
            if let Some(token) = TokenType::tokenize(c){
                token_array.push(token);
            }
        }
        return token_array;
    }

    fn get_loop_token_ptr(token_array:&Vec<TokenType>)->
                         (HashMap<u32,u32>,HashMap<u32,u32>){
        let mut start_end_map : HashMap<u32,u32> = HashMap::new();
        let mut end_start_map : HashMap<u32,u32> = HashMap::new();
        let mut start_ptr_stack : Vec<u32> = Vec::new();
        let mut ptr : u32 = 0;
        for token in token_array{
            match *token{
                TokenType::StartLoop => {
                    start_ptr_stack.push(ptr);
                },
                TokenType::EndLoop => {
                    if let Some(start_ptr) = start_ptr_stack.pop(){
                        start_end_map.insert(start_ptr, ptr);
                        end_start_map.insert(ptr, start_ptr);
                    }else{
                        panic!("Too many ']' tokens detected!");
                    }
                },
                _ => {}
            }
            ptr+=1;
        }
        if ! start_ptr_stack.is_empty(){
            panic!("Too many '[' tokens detected!");
        }
        return (start_end_map, end_start_map);
    }
}


struct BfInterpreter{
    token_array : Vec<TokenType>,
    token_ptr : u32,
    memory : Vec<u32>,
    memory_ptr: u16,
    input : VecDeque<char>,
    output : String,
    loop_start_end_token_ptr_map : HashMap<u32,u32>,
    loop_end_start_token_ptr_map : HashMap<u32,u32>,
}

impl BfInterpreter{
    fn init(src: &str, input: &str) -> BfInterpreter {
        let ta = TokenType::tokenize_from_array(src.chars().collect::<Vec<char>>());
        let (lsetpm,lestpm) = TokenType::get_loop_token_ptr(&ta);

        BfInterpreter{
            token_array : ta,
            token_ptr : 0,
            //Brainf*ck's number of memory cell is defined to be larger than 30,000.
            //So this program should reserve size of u16::max_value(), 
            //which is expected to be 2^16 = 65,536.
            memory : vec![0 ; u16::max_value() as usize],
            memory_ptr : 0,
            input : input.chars().collect(),
            output : String::from(""),
            loop_start_end_token_ptr_map : lsetpm,
            loop_end_start_token_ptr_map : lestpm,
        }
    }


    fn exec(&mut self){
        let token_array = &self.token_array;
        self.token_ptr = 0;
        while let Some(token) = token_array.get(self.token_ptr as usize){
            match *token{
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
                        &self.loop_start_end_token_ptr_map,
                        &mut self.token_ptr
                        );
                }
                TokenType::EndLoop => {
                    BfInterpreter::jump_loop_start_token_if_mem_not_0(
                        self.memory.get(self.memory_ptr as usize),
                        &self.loop_end_start_token_ptr_map,
                        &mut self.token_ptr
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
                                    loop_start_end_token_ptr_map:&HashMap<u32,u32>, 
                                    token_ptr : &mut u32){
        if let Some(val) = mem_val{
            if *val != 0{return;}
        }else{return;}

        if let Some(end_ptr) = loop_start_end_token_ptr_map.get(token_ptr){
            *token_ptr = *end_ptr;
        }else{
            panic!("no pair ']' token found.");
        }
    }
    //fn for token ']'.
    fn jump_loop_start_token_if_mem_not_0(mem_val:Option<&u32>, 
                                          loop_end_start_token_ptr_map:&HashMap<u32,u32>, 
                                          token_ptr : &mut u32){
        if let Some(val) = mem_val{
            if *val == 0{return;}
        }else{return;}

        if let Some(start_ptr) = loop_end_start_token_ptr_map.get(token_ptr){
            *token_ptr = *start_ptr;
        }else{
            panic!("no pair '[' token found.");
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
        //"+++++++++++++++++++++++++++++++++.";
    let input : &str = "";
    let mut  bf = BfInterpreter::init(src, input);
    //println!("start_end:{:?},end_start:{:?}",bf.loop_start_end_token_ptr_map,bf.loop_end_start_token_ptr_map);
    bf.exec();
    println!("{}",bf.output);
}
