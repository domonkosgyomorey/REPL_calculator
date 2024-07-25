use std::collections::{HashMap, VecDeque};
use lazy_static::lazy_static;
use std::io::prelude::*;
use std::num::Wrapping;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum TOKEN {
    PLUS,
    MINUS,
    MUL,
    LPAREN,
    RPAREN,
    NUMBER(Wrapping<u32>)
}

enum COMMAND {
    QUIT,
    HELP,
    EMPTY,
    EVAL
}

fn parse_token(c: char) -> Result<TOKEN, u32>{
    match c {
        '+' => Ok(TOKEN::PLUS),
        '-' => Ok(TOKEN::MINUS),
        '*' => Ok(TOKEN::MUL),
        '(' => Ok(TOKEN::LPAREN),
        ')' => Ok(TOKEN::RPAREN),
        _ => {
            if c.is_digit(10) {
                 Ok(TOKEN::NUMBER(Wrapping(c.to_digit(10).unwrap())))
            } else {
                 Err(UNKNOWN_TOKEN_ERROR) 
            }
        }
    }
}


fn clean_input(input:String) -> Result<Vec<TOKEN>, Vec<String>> {
    let mut tokens: Vec<TOKEN> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    for c in input.chars() {
        if c!=' ' {
            match parse_token(c) {
                Ok(t) => tokens.push(t),
                Err(e) => errors.push(format!("{}: {}", c, ERROR_MAP[&e]))    
            }
        }
    }

    if errors.len() > 0 { return Err(errors); }

    let mut i = 0;
    while i < tokens.len() {
        if let TOKEN::NUMBER(d) = tokens[i]{
            i += 1;
            if i >= tokens.len() {break}
            let mut number: Wrapping<u32> = d;
            while let TOKEN::NUMBER(d) = tokens[i] {
                number = number*Wrapping(10)+d;
                tokens.remove(i);
                if i >= tokens.len() {break}
            }
            i -= 1;
            tokens[i] = TOKEN::NUMBER(number);
        }
        i+=1;
    }
    return Ok(tokens)
}


fn return_numbers_for_bin_op(prev_token: Option<TOKEN>, next_token: TOKEN, operation: TOKEN) -> Result<[Wrapping<u32>;2], u32>{
    let mut result: [Wrapping<u32>; 2] = [Wrapping(0); 2];
    match prev_token {
        Some(TOKEN::NUMBER(num)) => { result[0] = num; },
        _ => { return Err(TOKEN_TO_ERROR_OFFSET[&operation]+LEFT_ARG_MISS_ERROR_IDX); }
    }
    match next_token {
        TOKEN::NUMBER(num) => { result[1] = num; },
        _ => { return Err(TOKEN_TO_ERROR_OFFSET[&operation]+RIGHT_ARG_MISS_ERROR_IDX); }
    }

    return Ok(result);
}

fn exec_bin_op(tokens: &mut Vec<TOKEN>, idx: &mut usize, prev_token: Option<TOKEN>, func: fn(Wrapping<u32>, Wrapping<u32>) -> Wrapping<u32>) -> Result<(), u32> {
    let operation: TOKEN = tokens[*idx].clone();
    if (*idx)+1 < tokens.len(){
        match return_numbers_for_bin_op(prev_token, tokens[(*idx)+1].clone(), operation) {
            Ok([left, right]) => {
                tokens[(*idx)-1] = TOKEN::NUMBER(func(left,right));
                tokens.remove(*(idx)+1);
                tokens.remove(*idx);

                // because end of the loop we increase i and we are on i-1
                *idx -= 1;
                Ok(())
            },
            Err(error_code) => { return Err(error_code); }
        }
    }else{
        return Err(TOKEN_TO_ERROR_OFFSET[&operation]+RIGHT_ARG_MISS_ERROR_IDX);
    }
}

fn eval_tokens(mut tokens :Vec<TOKEN>) -> Result<Wrapping<u32>, u32> {
    unsafe { LOG.push(format!("\nCurrent expr {:?}", tokens)); }

    let mut lparen_idxs:VecDeque<u32> = VecDeque::new();
    let mut prev_token: Option<TOKEN> = None;

    // paren check
    let mut i = 0;
    while i < tokens.len(){
        unsafe { LOG.push(format!("In P loop {:?} at {}", tokens[i], i)); }
        match tokens[i] {
            TOKEN::LPAREN => lparen_idxs.push_back(i as u32),
            TOKEN::RPAREN => {
                if lparen_idxs.is_empty(){
                    return Err(WRON_PAREN_ERROR);
                }else{
                    let idx:u32 = lparen_idxs.pop_back().unwrap()+1;
                    // evaluate inside arithmetic because we found the parenthesis around them
                    let sub_expr:Vec<TOKEN> = tokens.iter().skip(idx as usize).take(i-idx as usize).cloned().collect();
                    unsafe { LOG.push(format!("Sub Expression: {:?}", sub_expr)); }

                    match eval_tokens(sub_expr) {
                        Ok(result) => {tokens[(idx as usize)-1] = TOKEN::NUMBER(result);},
                        Err(msg) => { return Err(msg); }
                    }
                    unsafe { LOG.push(format!("Sub Expression Result: {:?}", tokens)); }

                    let mut j: usize = i;
                    while j >= idx as usize {
                        unsafe { LOG.push(format!("Token Deleted: {:?} at {}", tokens[j], j)); }
                        tokens.remove(j);
                        j -= 1;
                    }

                    unsafe { LOG.push(format!("AFTER DELETE: {:?}\n", tokens)); }

                    i = 0;
                    lparen_idxs = VecDeque::new();
                    continue;
                }
            },
            // I did this because its easier to remember to add new branch
            TOKEN::MINUS | TOKEN::MUL | TOKEN::NUMBER(_) | TOKEN::PLUS => {}
        };
        i+=1;
    }
    i = 0;
    while i < tokens.len() {
        unsafe { LOG.push(format!("In A loop {:?} at {}", tokens[i], i)); }
        match tokens[i] {
            TOKEN::PLUS => {
                match exec_bin_op(&mut tokens, &mut i, prev_token, |l,r| {l+r}) {
                    Err(error_code) => { return Err(error_code); },
                    Ok(_) => {}
                };
            },
            TOKEN::MINUS => {
                match exec_bin_op(&mut tokens, &mut i, prev_token, |l,r| {l-r}) {
                    Err(error_code) => { return Err(error_code); },
                    Ok(_) => {}
                };
            },
            TOKEN::MUL => {
                match exec_bin_op(&mut tokens, &mut i, prev_token, |l,r| {l*r}) {
                    Err(error_code) => { return Err(error_code); },
                    Ok(_) => {}
                };
            },
            // I did this because its easier to remember to add new branch
            TOKEN::LPAREN | TOKEN::RPAREN | TOKEN::NUMBER(_) => {}
        };
        prev_token = Some(tokens[i].clone());
        i += 1;
    }
    if let TOKEN::NUMBER(d) = tokens[0] {
        unsafe { LOG.push(format!("Result: {:?}", tokens)); }
        return Ok(d);
    }
    unreachable!()
}

fn eval(input: String){
    match clean_input(input) {
        Ok(xs) => match eval_tokens(xs) {
            Ok(res) => println!("=> {}", res),
            Err(error_code) => println!("\x1b[1;31mError: {}\x1b[0m", ERROR_MAP[&error_code])
        },
        Err(errors) => {
            for error in errors.iter() {
                println!("\x1b[1;31mError: {}\x1b[0m", error);
            }
        }
    }
}

static mut LOG:Vec<String> = Vec::new();

const UNKNOWN_TOKEN_ERROR:u32 = 1;
const WRON_PAREN_ERROR:u32 = 6;

const ADD_ERROR_OFFSET:u32 = 2;
const SUB_ERROR_OFFSET:u32 = 4;
const LEFT_ARG_MISS_ERROR_IDX: u32 = 0; 
const RIGHT_ARG_MISS_ERROR_IDX: u32 = 1; 

lazy_static! {
    static ref ERROR_MAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(1, "token cannot be parsed");

        // Add offset
        m.insert(2, "left argument is missing at an addition");
        m.insert(3, "right argument is missing at an addition");
        
        // Sub offset
        m.insert(4, "left argument is missing at a subtraction");
        m.insert(5, "right argument is missing at an subtraction");
        
        m.insert(6, "Wrong parenthesis found");
        return m;
    };

    static ref TOKEN_TO_ERROR_OFFSET: HashMap<TOKEN, u32> = {
        let mut m = HashMap::new();
        m.insert(TOKEN::PLUS, ADD_ERROR_OFFSET);
        m.insert(TOKEN::MINUS, SUB_ERROR_OFFSET);
        return m;
    };
}


fn print_help(){
    println!("========= HELP =========");
    println!("commands: \x1b[1;36m(quit, q)\x1b[0m");
    println!("evaluation:");
    println!("\x1b[1;36m\t((1)+11-3)\x1b[0m");
    println!("\x1b[1;32m\t=> 9\x1b[0m")
}

fn get_line() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => { input },
        Err(_) => panic!("Cannot read input")
    }
}

fn get_command(input: &str) -> COMMAND {
    let cmd = input.to_lowercase();
    if cmd=="quit" || cmd=="q" { return COMMAND::QUIT; }
    if cmd=="help" || cmd=="h" { return COMMAND::HELP; }
    if cmd=="" { return COMMAND::EMPTY; }
    return COMMAND::EVAL;
}

fn main() -> std::io::Result<()>{
    loop {

        print!("$ ");
        std::io::stdout().flush().unwrap();
        let input: String = get_line().trim().to_string();
        match get_command(&input) {
            COMMAND::EVAL => eval(input),
            COMMAND::HELP => print_help(),
            COMMAND::EMPTY => { continue; }
            COMMAND::QUIT => { break; }
        }

        let mut fp = std::fs::OpenOptions::new().write(true).truncate(true).open("log.txt")?;
        unsafe {
            for line in LOG.iter() {
                fp.write(line.as_bytes())?;
                fp.write("\n".as_bytes())?;
            }
        }
    }
    Ok(())
}
