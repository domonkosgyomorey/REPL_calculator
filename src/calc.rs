use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;
use std::num::Wrapping;
use std::str::Chars;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum TOKEN {
    PLUS(usize),
    MINUS(usize),
    MUL(usize),
    DIV(usize),
    LPAREN(usize),
    RPAREN(usize),
    NUMBER(Wrapping<u32>, usize),
}

#[derive(Debug, Clone)]
enum ASTNode {
    Number(Wrapping<u32>),
    Operator { op: TOKEN, left: Box<ASTNode>, right: Box<ASTNode>}
}

#[derive(PartialEq)]
enum Associativity {
    LEFT,
    NOT,
}

impl ASTNode {
    fn eval(&self) -> Result<Wrapping<u32>, (u32, Option<usize>)> {
        match self {
            ASTNode::Number(val) => Ok(*val),
            ASTNode::Operator { op, left, right } => {
                let (maybe_left_val, maybe_right_val): (Result<Wrapping<u32>, (u32, Option<usize>)>, Result<Wrapping<u32>, (u32, Option<usize>)>) = rayon::join(|| left.eval(), || right.eval());
                let lval = match maybe_left_val {
                    Err(err_code) => { return Err(err_code); },
                    Ok(v) => v
                };
                let rval = match maybe_right_val {
                    Err(err_code) => { return Err(err_code); },
                    Ok(v) => v
                };
                match op {
                    TOKEN::PLUS(_) => Ok(lval + rval),
                    TOKEN::MINUS(_) => Ok(lval - rval),
                    TOKEN::MUL(_) => Ok(lval * rval),
                    TOKEN::DIV(char_idx) => {
                        if rval == Wrapping(0) { return Err((DIVIDE_BY_ZERO_ERROR, Some(*char_idx))); }
                        return Ok(lval / rval);
                    },
                    _ => unreachable!(),
                }
            }
        }
    }
}

static mut LOG:Vec<String> = Vec::new();

const UNKNOWN_TOKEN_ERROR: u32 = 1;
const WRON_PAREN_ERROR: u32 = 2;
const DIVIDE_BY_ZERO_ERROR: u32 = 3;
const NO_RESULT_ERROR:u32 = 4;
const ARG_MISS_ERROR:u32 = 9;

lazy_static! {
    static ref ERROR_MAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(UNKNOWN_TOKEN_ERROR, "Token cannot be parsed");
        m.insert(WRON_PAREN_ERROR, "Wrong parenthesis found");
        m.insert(DIVIDE_BY_ZERO_ERROR, "Divided by zero");
        m.insert(NO_RESULT_ERROR, "No Result");

        m.insert(ARG_MISS_ERROR, "Argumentum is missing");
        return m;
    };
}

pub fn write_log(file_path: &'static str) -> Result<(), std::io::Error>{
    let mut fp = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)?;

    unsafe {
        for line in LOG.iter() {
            fp.write(line.as_bytes())?;
            fp.write("\n".as_bytes())?;
        }
    }
    Ok(())
}

pub fn eval(input: String, output: &mut Vec<String>) {
    if !is_parens_correct
(input.chars()) { output.push(format_error(ERROR_MAP[&WRON_PAREN_ERROR], None)); return; }
    match lexer(input) {
        Ok(tokens) => {
            unsafe { LOG.push(format!("Tokens {:?}", tokens)); }
            let infixed_tokens = shunting_yard_algorithm(tokens.clone());
            unsafe { LOG.push(format!("Infixed syntax {:?}", infixed_tokens)); }
            let expression_tree = generate_ast(infixed_tokens);
            unsafe { LOG.push(format!("Expression tree {:?}", expression_tree)); }
            match expression_tree {
                Ok(res) => output.push(match res.eval() {
                    Ok(res) => {
                        unsafe { LOG.push(format!("{:?}", res)); }
                        unsafe { LOG.push("".to_string()); }
                        format_result(res)
                    },
                    Err((err_code, err_idx)) => format_error(ERROR_MAP[&err_code], err_idx)
                }),
                Err((err_code, err_idx)) => output.push(format_error(ERROR_MAP[&err_code], err_idx))
            }
        },
        Err(errors) => {
            for error in errors.iter() {
                output.push(format!("\x1b[1;31mError: {}\x1b[0m", error));
            }
        }
    }
}

fn format_error<T: Debug>(msg: T, char_idx: Option<usize>) -> String{
    let maybe_char_idx: String = match char_idx {
        Some(ci) => format!(" at {}", ci),
        None => "".to_string(),
    };
    return format!("\x1b[1;31mError: {:?}{}\x1b[0m", msg, maybe_char_idx);
}

fn format_result<T: Debug>(msg: T) -> String{
    return format!("\x1b[1;32m=> {:?}\x1b[0m", msg);
}

fn op_precedence(token: TOKEN) -> u32 {
    match token {
        TOKEN::DIV(_) | TOKEN::MUL(_) => 2,
        TOKEN::PLUS(_) | TOKEN::MINUS(_) => 1,
        TOKEN::LPAREN(_) | TOKEN::RPAREN(_) | TOKEN::NUMBER(_, _) => 0
    }
}

fn op_associative(token: TOKEN) -> Associativity {
    match token {
        TOKEN::LPAREN(_) | TOKEN::RPAREN(_) => Associativity::NOT,
        TOKEN::NUMBER(_, _) => unreachable!(),
        _ => Associativity::LEFT
    }
}

fn lexer(input: String) -> Result<Vec<TOKEN>, Vec<String>> {
    let mut tokens: Vec<TOKEN> = Vec::new();
    let mut errors: Vec<String> = Vec::new();
    let mut i: usize = 0;
    let mut nc: char;
    while i < input.len() {
        nc = input.chars().nth(i).unwrap();
        if nc != ' ' {
            match nc {
                '+' => tokens.push(TOKEN::PLUS(i)),
                '-' => tokens.push(TOKEN::MINUS(i)),
                '*' => tokens.push(TOKEN::MUL(i)),
                '(' => tokens.push(TOKEN::LPAREN(i)),
                ')' => tokens.push(TOKEN::RPAREN(i)),
                '/' => tokens.push(TOKEN::DIV(i)),
                _ => {
                    if nc.is_digit(10) {
                        let mut number:String = String::new();
                        number.push(nc);
                        while i+1<input.len() {
                            nc = input.chars().nth(i+1).unwrap();
                            if nc.is_digit(10) {
                                number.push(input.chars().nth(i+1).unwrap());
                            }else if !(nc.is_digit(10) || nc == ' ') {
                                break;
                            }
                            i += 1;
                        }
                        tokens.push(TOKEN::NUMBER(Wrapping(number.parse().unwrap()), i));
                    } else { errors.push(format!("{}: {}", nc, ERROR_MAP[&UNKNOWN_TOKEN_ERROR])); }
                }
            }
        }
        i+=1;
    }

    if errors.len() > 0 { return Err(errors); }
    return Ok(tokens);
}

fn is_parens_correct(chrs: Chars) -> bool {
    let mut paren_stack: VecDeque<char> = VecDeque::new();
    for chr in chrs {
        match chr {
            '(' => paren_stack.push_back(chr),
            ')' => {
                if paren_stack.is_empty() { return false; }
                _ = paren_stack.pop_back();
            },
            _ => {}
        }
    }
    return paren_stack.is_empty();
}

fn shunting_yard_algorithm(tokens: Vec<TOKEN>) -> VecDeque<TOKEN> {
    let mut output: VecDeque<TOKEN> = VecDeque::new();
    let mut operators: Vec<TOKEN> = Vec::new();

    for token in tokens {
        match token {
            TOKEN::PLUS(_) | TOKEN::MINUS(_) | TOKEN::MUL(_) | TOKEN::DIV(_) => {
                while let Some(op) = operators.last() {
                    let o1 = token.clone();
                    let o2 = op.clone();
                    if let TOKEN::LPAREN(_) = o2 {
                        break;
                    }
                    if op_precedence(o2.clone()) > op_precedence(o1.clone()) ||
                        (op_precedence(o1.clone()) == op_precedence(o2.clone()) && op_associative(o1) == Associativity::LEFT){
                            output.push_back(operators.pop().unwrap());
                    }else{
                        break;
                    }
                }
                operators.push(token);
            },
            TOKEN::LPAREN(_) => operators.push(token),
            TOKEN::RPAREN(_) => {
                while let Some(op) = operators.last() {
                    if let TOKEN::LPAREN(_) = *op { 
                        operators.pop();
                        break;
                    }
                    output.push_back(operators.pop().unwrap());
                }
            },
            TOKEN::NUMBER(_, _) => output.push_back(token)
        }
    }

    while let Some(op) = operators.pop() {
        output.push_back(op);
    }

    return output;
    
}

fn generate_ast(tokens: VecDeque<TOKEN>) -> Result<ASTNode, (u32, Option<usize>)> {
    let mut stack: Vec<ASTNode> = Vec::new();

    for token in tokens {
        match token {
            TOKEN::PLUS(char_idx) | TOKEN::MINUS(char_idx) | TOKEN::MUL(char_idx) | TOKEN::DIV(char_idx) => {
                let left = Box::new(match stack.pop(){
                    Some(v) => v,
                    None => { return Err((ARG_MISS_ERROR, Some(char_idx))); }
                });
                let right = Box::new(match stack.pop() {
                    Some(v) => v,
                    None => { return Err((ARG_MISS_ERROR, Some(char_idx))); }
                });
                stack.push(ASTNode::Operator { op: token, left: right, right: left })
            },
            TOKEN::NUMBER(num, _) => {
                stack.push(ASTNode::Number(num));
            },
            _ => {}
        }
    }
    return Ok(match stack.pop(){
        Some(v) => v,
        None => { return Err((NO_RESULT_ERROR, None)); }
    });
}