use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;


#[derive(Debug, PartialEq, Eq, Clone)]
enum TOKEN {
    PLUS,
    MINUS,
    LPAREN,
    RPAREN,
    NUMBER(u32)
}

fn parse_token(c: char) -> Result<TOKEN, &'static str>{
    match c {
        '+' => Ok(TOKEN::PLUS),
        '-' => Ok(TOKEN::MINUS),
        '(' => Ok(TOKEN::LPAREN),
        ')' => Ok(TOKEN::RPAREN),
        _ => {
            if c.is_digit(10) {
                 Ok(TOKEN::NUMBER(c.to_digit(10).unwrap()))
            } else {
                 Err("Token not parseable") 
            }
        }
    }
}

fn clean_input(input: &str) -> Vec<TOKEN> {
    let mut tokens: Vec<TOKEN> = input.chars().filter(|&c| c!=' ').map(|c|
        match parse_token(c) {
            Ok(t) => t,
            Err(e) => panic!("{}: {}", c, e)
        }).collect();

    let mut i = 0;
    while i < tokens.len() {
        if let TOKEN::NUMBER(d) = tokens[i]{
            i += 1;
            if i >= tokens.len() {break}
            let mut number: u32 = d;
            let mut digit_num:u32 = d;
            while let TOKEN::NUMBER(d) = tokens[i] {
                number += (10 as u32).pow(digit_num)*d;
                digit_num += 1;
                tokens.remove(i);
                if i >= tokens.len() {break}
            }
            i -= 1;
            tokens[i] = TOKEN::NUMBER(number);
        }
        i+=1;
    }
    return tokens
}

fn eval_tokens(mut tokens :Vec<TOKEN>) -> u32 {
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
                    panic!("Wrong paranthesis found");
                }else{
                    let idx:u32 = lparen_idxs.pop_back().unwrap()+1;
                    // evaluate inside arithmetic because we found the parenthesis around them
                    let sub_expr:Vec<TOKEN> = tokens.iter().skip(idx as usize).take(i-idx as usize).cloned().collect();
                    unsafe { LOG.push(format!("Sub Expression: {:?}", sub_expr)); }

                    tokens[(idx as usize)-1] = TOKEN::NUMBER(eval_tokens(sub_expr));
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
            _ => {}
        };
        i+=1;
    }
    i = 0;
    while i < tokens.len() {
        unsafe { LOG.push(format!("In A loop {:?} at {}", tokens[i], i)); }
        match tokens[i] {
            TOKEN::PLUS => {
                let mut left;
                match prev_token {
                    Some(TOKEN::NUMBER(p)) => {
                        left = p;
                    },
                    None => panic!("left argument is missing at a plus sign"),
                    _ => panic!("left argument is wrong at a plus sign")
                }
                if i+1 < tokens.len() {
                    if let TOKEN::NUMBER(d) = tokens[i+1] {
                        tokens[i-1] = TOKEN::NUMBER(left+d);
                        tokens.remove(i+1);
                        tokens.remove(i);

                        // because end of the loop we increase i and we are on i-1
                        i -= 1;
                    }else{
                        panic!("right argument is missing at a plus sign");
                    }
                }else{
                    panic!("right argument is missing at a plus sign");
                }
            },
            _ => {}
        };
        prev_token = Some(tokens[i].clone());
        i += 1;
    }
    if let TOKEN::NUMBER(d) = tokens[0] {
        unsafe { LOG.push(format!("Result: {:?}", tokens)); }
        return d;
    }
    unreachable!()
}

fn get_line(msg: &str) -> String {
    let mut input = String::new();
    
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => { input },
        Err(_) => panic!("Cannot read input")
    }
}

static mut LOG:Vec<String> = Vec::new(); 

fn main() -> std::io::Result<()>{
    loop {
        let input: String = get_line(">");
        println!("=> {}", eval_tokens(clean_input(input.trim())));
        let mut fp = std::fs::OpenOptions::new().write(true).open("./log.txt")?;
        unsafe {
            for line in LOG.iter() {
                fp.write(line.as_bytes())?;
                fp.write("\n".as_bytes())?;
            }
        }
    }
}
