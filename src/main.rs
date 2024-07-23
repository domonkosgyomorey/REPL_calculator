use std::collections::VecDeque;
use std::io::prelude::*;


#[derive(Debug, PartialEq, Eq, Clone)]
enum TOKEN {
    PLUS,
    MINUS,
    LPAREN,
    RPAREN,
    NUMBER(u32)
}

enum COMMAND {
    QUIT,
    HELP,
    EVAL
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

fn clean_input(input:String) -> Result<Vec<TOKEN>, Vec<String>> {
    let mut tokens: Vec<TOKEN> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    for c in input.chars() {
        if c!=' ' {
            match parse_token(c) {
                Ok(t) => tokens.push(t),
                Err(e) => errors.push(format!("{}: {}", c, e))    
            }
        }
    }

    if errors.len() > 0 {
        return Err(errors);
    }

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
    return Ok(tokens)
}

fn eval_tokens(mut tokens :Vec<TOKEN>) -> Result<u32, &'static str> {
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
                    return Err("Wrong paranthesis found");
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
            _ => {}
        };
        i+=1;
    }
    i = 0;
    while i < tokens.len() {
        unsafe { LOG.push(format!("In A loop {:?} at {}", tokens[i], i)); }
        match tokens[i] {
            TOKEN::PLUS => {
                let left;
                match prev_token {
                    Some(TOKEN::NUMBER(p)) => {
                        left = p;
                    },
                    None => { return Err("left argument is missing at an addition"); },
                    _ => { return Err("left argument is wrong at an addition"); }
                }
                if i+1 < tokens.len() {
                    if let TOKEN::NUMBER(d) = tokens[i+1] {
                        tokens[i-1] = TOKEN::NUMBER(left+d);
                        tokens.remove(i+1);
                        tokens.remove(i);

                        // because end of the loop we increase i and we are on i-1
                        i -= 1;
                    }else{
                        return Err("right argument is missing at an addition");
                    }
                }else{
                    return Err("right argument is missing at an addition");
                }
            },
            TOKEN::MINUS => {
                let left;
                match prev_token {
                    Some(TOKEN::NUMBER(p)) => {
                        left = p;
                    },
                    None => { return Err("left argument is missing at a substraction"); },
                    _ => { return Err("left argument is wrong at a substraction"); }
                }
                if i+1 < tokens.len() {
                    if let TOKEN::NUMBER(d) = tokens[i+1] {
                        if d > left {
                            return Err("Negative numbers not supported");
                        }
                        tokens[i-1] = TOKEN::NUMBER(left-d);
                        tokens.remove(i+1);
                        tokens.remove(i);

                        // because end of the loop we increase i and we are on i-1
                        i -= 1;
                    }else{
                        return Err("right argument is missing at a substraction");
                    }
                }else{
                    return Err("right argument is missing at a substraction");
                }
            },
            _ => {}
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
    return COMMAND::EVAL;
}

fn eval(input: String){
    match clean_input(input) {
        Ok(xs) => match eval_tokens(xs) {
            Ok(res) => println!("=> {}", res),
            Err(msg) => println!("\x1b[1;31mError: {}\x1b[0m", msg)
        },
        Err(errors) => {
            for error in errors.iter() {
                println!("\x1b[1;31mError: {}\x1b[0m", error);
            }
        }
    }
}

fn print_help(){
    println!("========= HELP =========");
    println!("commands: \x1b[1;36m(quit, q)\x1b[0m");
    println!("evaluation:");
    println!("\x1b[1;36m\t((1)+11-3)\x1b[0m");
    println!("\x1b[1;32m\t=> 9\x1b[0m")
}

static mut LOG:Vec<String> = Vec::new(); 

fn main() -> std::io::Result<()>{
    loop {

        print!("$ ");
        std::io::stdout().flush().unwrap();
        let input: String = get_line().trim().to_string();
        match get_command(&input) {
            COMMAND::EVAL => eval(input),
            COMMAND::HELP => print_help(),
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
