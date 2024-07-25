use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};
use std::num::Wrapping;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum TOKEN {
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    NUMBER(Wrapping<u32>),
}

pub static mut LOG: Vec<String> = Vec::new();

const UNKNOWN_TOKEN_ERROR: u32 = 1;
const WRON_PAREN_ERROR: u32 = 2;
const DIVIDE_BY_ZERO_ERROR: u32 = 3;
const NO_RESULT_ERROR:u32 = 4;

const SEPCIFIC_ERROR_START:u32 = 5;
// In this case this is 2 because LEFT and RIGHT arg missing
const SPECIFIC_ERRORS_FIELD_LEN:u32 = 2;

const ADD_ERROR_OFFSET: u32 = SEPCIFIC_ERROR_START;
const SUB_ERROR_OFFSET: u32 = SEPCIFIC_ERROR_START+SPECIFIC_ERRORS_FIELD_LEN;
const LEFT_ARG_MISS_ERROR_IDX: u32 = 0;
const RIGHT_ARG_MISS_ERROR_IDX: u32 = 1;

lazy_static! {
    static ref ERROR_MAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(UNKNOWN_TOKEN_ERROR, "Token cannot be parsed");
        m.insert(WRON_PAREN_ERROR, "Wrong parenthesis found");
        m.insert(DIVIDE_BY_ZERO_ERROR, "Divided by zero");
        m.insert(NO_RESULT_ERROR, "No Result");

        // Add offset
        m.insert(ADD_ERROR_OFFSET+LEFT_ARG_MISS_ERROR_IDX, "Left argument is missing at an addition");
        m.insert(ADD_ERROR_OFFSET+RIGHT_ARG_MISS_ERROR_IDX, "Right argument is missing at an addition");

        // Sub offset
        m.insert(SUB_ERROR_OFFSET+LEFT_ARG_MISS_ERROR_IDX, "Left argument is missing at a subtraction");
        m.insert(SUB_ERROR_OFFSET+RIGHT_ARG_MISS_ERROR_IDX, "Right argument is missing at an subtraction");

        return m;
    };

    static ref TOKEN_TO_ERROR_OFFSET: HashMap<TOKEN, u32> = {
        let mut m = HashMap::new();
        m.insert(TOKEN::PLUS, ADD_ERROR_OFFSET);
        m.insert(TOKEN::MINUS, SUB_ERROR_OFFSET);
        return m;
    };
}

pub fn eval(input: String, output: &mut Vec<String>) {
    if !is_tokens_correct(input.chars()) { output.push(format!("{}", ERROR_MAP[&WRON_PAREN_ERROR])); return; }
    match clean_input(input) {
        Ok(xs) => match eval_tokens(xs) {
            Ok(res) => {
                match res {
                    Some(res) => output.push(format!("=> {}", res)),
                    None => output.push(format!("\x1b[1;No Result\x1b[0m"))
                }
            }
            Err(error_code) => output.push(format!("\x1b[1;31mError: {}\x1b[0m", ERROR_MAP[&error_code])),
        },
        Err(errors) => {
            for error in errors.iter() {
                output.push(format!("\x1b[1;31mError: {}\x1b[0m", error));
            }
        }
    }
}

fn clean_input(input: String) -> Result<Vec<TOKEN>, Vec<String>> {
    let mut tokens: Vec<TOKEN> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    for c in input.chars() {
        if c != ' ' {
            match parse_token(c) {
                Ok(t) => tokens.push(t),
                Err(e) => errors.push(format!("{}: {}", c, ERROR_MAP[&e])),
            }
        }
    }

    if errors.len() > 0 {
        return Err(errors);
    }

    let mut i = 0;
    while i < tokens.len() {
        if let TOKEN::NUMBER(d) = tokens[i] {
            i += 1;
            if i >= tokens.len() {
                break;
            }
            let mut number: Wrapping<u32> = d;
            while let TOKEN::NUMBER(d) = tokens[i] {
                number = number * Wrapping(10) + d;
                tokens.remove(i);
                if i >= tokens.len() {
                    break;
                }
            }
            i -= 1;
            tokens[i] = TOKEN::NUMBER(number);
        }
        i += 1;
    }
    return Ok(tokens);
}

fn parse_token(c: char) -> Result<TOKEN, u32> {
    match c {
        '+' => Ok(TOKEN::PLUS),
        '-' => Ok(TOKEN::MINUS),
        '*' => Ok(TOKEN::MUL),
        '(' => Ok(TOKEN::LPAREN),
        ')' => Ok(TOKEN::RPAREN),
        '/' => Ok(TOKEN::DIV),
        _ => {
            if c.is_digit(10) {
                Ok(TOKEN::NUMBER(Wrapping(c.to_digit(10).unwrap())))
            } else { Err(UNKNOWN_TOKEN_ERROR) }
        }
    }
}

fn is_tokens_correct(chrs: Chars) -> bool {
    let mut paren_stack: VecDeque<char> = VecDeque::new();
    for chr in chrs {
        match chr {
            '(' => paren_stack.push_back(chr),
            ')' => {_ = paren_stack.pop_back();},
            _ => {}
        }
    }
    return paren_stack.is_empty();
}

fn eval_tokens(mut tokens: Vec<TOKEN>) -> Result<Option<Wrapping<u32>>, u32> {
    unsafe { LOG.push(format!("\nCurrent expr {:?}", tokens)); }

    let mut lparen_idxs: VecDeque<u32> = VecDeque::new();
    let mut prev_token: Option<TOKEN> = None;

    // paren check
    let mut i = 0;
    while i < tokens.len() {
        unsafe { LOG.push(format!("In Parenthesis loop {:?} at {}", tokens[i], i)); }
        match tokens[i] {
            TOKEN::LPAREN => lparen_idxs.push_back(i as u32),
            TOKEN::RPAREN => {
                if lparen_idxs.is_empty() {
                    return Err(WRON_PAREN_ERROR);
                } else {
                    let idx: u32 = lparen_idxs.pop_back().unwrap() + 1;
                    // evaluate inside arithmetic because we found the parenthesis around them
                    let sub_expr: Vec<TOKEN> = tokens.iter().skip(idx as usize).take(i - idx as usize).cloned().collect();
                    unsafe { LOG.push(format!("Sub Expression: {:?}", sub_expr)); }

                    match eval_tokens(sub_expr) {
                        Ok(result) => { 
                            match result {
                                Some(result) => tokens[(idx as usize) - 1] = TOKEN::NUMBER(result),
                                None => { return Err(NO_RESULT_ERROR); }
                            }
                        }
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
            }
            // I did this because its easier to remember to add new branch
            TOKEN::MINUS | TOKEN::MUL | TOKEN::NUMBER(_) | TOKEN::PLUS | TOKEN::DIV => {}
        };
        i += 1;
    }
    i = 0;
    while i < tokens.len() {
        unsafe { LOG.push(format!("In Arithmetic loop {:?} at {}", tokens[i], i)); }
        if let Err(error_code) = match tokens[i] {
            TOKEN::PLUS => exec_bin_op(&mut tokens, &mut i, prev_token, |l, r| Ok(l + r)),
            TOKEN::MINUS => exec_bin_op(&mut tokens, &mut i, prev_token, |l, r| Ok(l - r)),
            TOKEN::MUL => exec_bin_op(&mut tokens, &mut i, prev_token, |l, r| Ok(l * r)),
            TOKEN::DIV => exec_bin_op(&mut tokens, &mut i, prev_token, |l, r| {
                if r.0 == 0 {
                    return Err(DIVIDE_BY_ZERO_ERROR);
                }
                Ok(l / r)
            }),
            // I did this because its easier to remember to add new branch
            TOKEN::LPAREN | TOKEN::RPAREN | TOKEN::NUMBER(_) => Ok(()),
        } { return Err(error_code); };

        prev_token = Some(tokens[i].clone());
        i += 1;
    }
    if tokens.len() == 0 {
        return Ok(None);
    } else if let TOKEN::NUMBER(d) = tokens[0] {
        unsafe {
            LOG.push(format!("Result: {:?}", tokens));
        }
        return Ok(Some(d));
    }
    unreachable!()
}

fn return_numbers_for_bin_op( prev_token: Option<TOKEN>, next_token: TOKEN, operation: TOKEN) -> Result<[Wrapping<u32>; 2], u32> {
    let mut result: [Wrapping<u32>; 2] = [Wrapping(0); 2];
    match prev_token {
        Some(TOKEN::NUMBER(num)) => { result[0] = num; }
        _ => { return Err(TOKEN_TO_ERROR_OFFSET[&operation] + LEFT_ARG_MISS_ERROR_IDX); }
    }
    match next_token {
        TOKEN::NUMBER(num) => { result[1] = num; }
        _ => { return Err(TOKEN_TO_ERROR_OFFSET[&operation] + RIGHT_ARG_MISS_ERROR_IDX); }
    }

    return Ok(result);
}

fn exec_bin_op(tokens: &mut Vec<TOKEN>, idx: &mut usize, prev_token: Option<TOKEN>, func: fn(Wrapping<u32>, Wrapping<u32>) -> Result<Wrapping<u32>, u32>,) -> Result<(), u32> {
    let operation: TOKEN = tokens[*idx].clone();
    if (*idx) + 1 < tokens.len() {
        match return_numbers_for_bin_op(prev_token, tokens[(*idx) + 1].clone(), operation) {
            Ok([left, right]) => {
                match func(left, right) {
                    Ok(result) => {
                        tokens[(*idx) - 1] = TOKEN::NUMBER(result);
                    }
                    Err(error_code) => {
                        return Err(error_code);
                    }
                }

                tokens.remove(*(idx) + 1);
                tokens.remove(*idx);

                // because end of the loop we increase i and we are on i-1
                *idx -= 1;
                Ok(())
            }
            Err(error_code) => {
                return Err(error_code);
            }
        }
    } else {
        return Err(TOKEN_TO_ERROR_OFFSET[&operation] + RIGHT_ARG_MISS_ERROR_IDX);
    }
}
