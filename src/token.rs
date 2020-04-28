use std::collections::HashMap;
use std::collections::VecDeque;

pub enum Token {
    Inc,
    Dec,
    IncPtr,
    DecPtr,
    StartLoop,
    EndLoop,
    Input,
    Output,
}

impl Token {
    fn tokenize(c: char) -> Option<Token> {
        match c {
            '+' => Some(Token::Inc),
            '-' => Some(Token::Dec),
            '>' => Some(Token::IncPtr),
            '<' => Some(Token::DecPtr),
            '[' => Some(Token::StartLoop),
            ']' => Some(Token::EndLoop),
            ',' => Some(Token::Input),
            '.' => Some(Token::Output),
            _ => None,
        }
    }

    pub fn tokenize_from_array(char_array: Vec<char>) -> Vec<Token> {
        let mut token_array: Vec<Token> = Vec::new();
        for token in char_array.iter().filter_map(|c| Token::tokenize(*c)) {
            token_array.push(token);
        }
        return token_array;
    }

    pub fn get_loop_token_ptr(token_array: &Vec<Token>) -> (HashMap<u32, u32>, HashMap<u32, u32>) {
        let mut start_end_map: HashMap<u32, u32> = HashMap::new();
        let mut end_start_map: HashMap<u32, u32> = HashMap::new();
        let mut start_ptr_stack: Vec<u32> = Vec::new();
        let mut ptr: u32 = 0;
        for token in token_array {
            match *token {
                Token::StartLoop => {
                    start_ptr_stack.push(ptr);
                }
                Token::EndLoop => {
                    if let Some(start_ptr) = start_ptr_stack.pop() {
                        start_end_map.insert(start_ptr, ptr);
                        end_start_map.insert(ptr, start_ptr);
                    } else {
                        panic!("Too many `]` tokens detected!");
                    }
                }
                _ => {}
            }
            ptr += 1;
        }
        if !start_ptr_stack.is_empty() {
            panic!("Too many `[` tokens detected!");
        }
        return (start_end_map, end_start_map);
    }

    //fn for token `+`.
    pub fn inc_mem_val(memory: &mut Vec<u32>, memory_ptr: u16) {
        if let Some(val) = memory.get_mut(memory_ptr as usize) {
            *val += 1;
        }
    }
    //fn for token `-`.
    pub fn dec_mem_val(memory: &mut Vec<u32>, memory_ptr: u16) {
        if let Some(val) = memory.get_mut(memory_ptr as usize) {
            *val -= 1;
        }
    }
    //fn for token `>`.
    pub fn inc_mem_ptr(memory_ptr: &mut u16) {
        *memory_ptr += 1;
    }
    //fn for token `<`.
    pub fn dec_mem_ptr(memory_ptr: &mut u16) {
        *memory_ptr -= 1;
    }
    //fn for token `[`.
    pub fn jump_loop_end_token_if_mem_0(
        mem_val: Option<&u32>,
        loop_start_end_token_ptr_map: &HashMap<u32, u32>,
        token_ptr: &mut u32,
    ) {
        if let Some(val) = mem_val {
            if *val != 0 {
                return;
            }
        } else {
            return;
        }

        if let Some(end_ptr) = loop_start_end_token_ptr_map.get(token_ptr) {
            *token_ptr = *end_ptr;
        } else {
            panic!("no pair `]` token found.");
        }
    }
    //fn for token `]`.
    pub fn jump_loop_start_token_if_mem_not_0(
        mem_val: Option<&u32>,
        loop_end_start_token_ptr_map: &HashMap<u32, u32>,
        token_ptr: &mut u32,
    ) {
        if let Some(val) = mem_val {
            if *val == 0 {
                return;
            }
        } else {
            return;
        }

        if let Some(start_ptr) = loop_end_start_token_ptr_map.get(token_ptr) {
            *token_ptr = *start_ptr;
        } else {
            panic!("no pair `[` token found.");
        }
    }
    //fn for token `,`.
    pub fn put_char_from_input_to_mem(
        input_char_array: &mut VecDeque<char>,
        memory: &mut Vec<u32>,
        memory_ptr: u16,
    ) {
        if let Some(val) = memory.get_mut(memory_ptr as usize) {
            if let Some(c) = input_char_array.pop_front() {
                *val = u32::from(c);
            }
        }
    }
    //fn for token `.`.
    pub fn join_output_char_to_str(output_char: Option<char>, output_str: &mut String) {
        if let Some(c) = output_char {
            output_str.push(c);
        }
    }
}