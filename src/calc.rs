use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;
use std::num::Wrapping;
use std::str::Chars;
use std::sync::Mutex;

type ErrorMsg = (String, Option<usize>);
type ErrorCode = (u32, Option<usize>);
type CalcNumber = Wrapping<u32>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum TOKEN {
    PLUS(usize),
    MINUS(usize),
    MUL(usize),
    DIV(usize),
    POW(usize),
    FACT(usize),
    SQRT(usize),
    MOD(usize),

    EQUAL(usize),
    NOT(usize),
    GT(usize),
    GE(usize),
    LT(usize),
    LE(usize),
    NEQUAL(usize),
    AND(usize),
    OR(usize),
    XOR(usize),

    BAND(usize),
    BOR(usize),
    BXOR(usize),

    ASSIGN(usize),
    VAR(usize),
    
    LPAREN(usize),
    RPAREN(usize),
    NUMBER(CalcNumber, usize),
}

#[derive(Debug, Clone)]
enum ASTNode {
    Number(CalcNumber),
    ULOperator { op: TOKEN, left: Box<ASTNode>},
    UROperator { op: TOKEN, right: Box<ASTNode>},
    BOperator { op: TOKEN, left: Box<ASTNode>, right: Box<ASTNode>}
}

#[derive(PartialEq)]
enum Associativity {
    RIGHT,
    LEFT,
    NOT,
}

impl ASTNode {
    fn eval(&self) -> Result<CalcNumber, ErrorCode> {
        match self {
            ASTNode::Number(val) => Ok(*val),
            ASTNode::BOperator { op, left, right } => {
                let (maybe_left_val, maybe_right_val): (Result<CalcNumber, ErrorCode>, Result<CalcNumber, ErrorCode>) = rayon::join(|| left.eval(), || right.eval());
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
                    TOKEN::POW(_) => Ok(Wrapping(lval.0.wrapping_pow(rval.0))),
                    TOKEN::MOD(char_idx) => {
                        if rval.0 == 0 { return Err((DIVIDE_BY_ZERO_ERROR, Some(*char_idx))); }
                        return Ok(Wrapping(lval.0%rval.0));
                    },
                    TOKEN::AND(_) => Ok(to_calc_num(to_bool(lval) && to_bool(rval))),
                    TOKEN::OR(_) => Ok(to_calc_num(to_bool(lval) || to_bool(rval))),
                    TOKEN::XOR(_) => Ok(to_calc_num(to_bool(lval)^to_bool(rval))),
                    TOKEN::BOR(_) => Ok(lval|rval),
                    TOKEN::BAND(_) => Ok(lval&rval),
                    TOKEN::BXOR(_) => Ok(lval^rval),
                    TOKEN::EQUAL(_) => Ok(to_calc_num(lval==rval)),
                    TOKEN::GT(_) => Ok(to_calc_num(lval>rval)),
                    TOKEN::GE(_) => Ok(to_calc_num(lval>=rval)),
                    TOKEN::LT(_) => Ok(to_calc_num(lval<rval)),
                    TOKEN::LE(_) => Ok(to_calc_num(lval<=rval)),
                    TOKEN::NEQUAL(_) => Ok(to_calc_num(lval!=rval)),
                    _ => unreachable!(),
                }
            },
            ASTNode::ULOperator { op, left } => {
                let meybe_left: Result<CalcNumber, ErrorCode> = left.eval();
                let lval = match meybe_left {
                    Err(err_code) => { return Err(err_code); },
                    Ok(v) => v
                };
                match op {
                    TOKEN::FACT(_) => Ok(factorial(lval)),
                    _ => unreachable!(),
                }
            },
            ASTNode::UROperator { op, right } => {
                let maybe_right_val:Result<CalcNumber, ErrorCode> = right.eval();
                let rval = match maybe_right_val {
                    Err(err_code) => { return Err(err_code); },
                    Ok(v) => v
                };
                match op {
                    TOKEN::SQRT(_) => Ok(Wrapping(f64::from(rval.0).sqrt() as u32)),
                    TOKEN::NOT(_) => Ok(to_calc_num(!to_bool(rval))),
                    _ => unreachable!()
                }
            }
        }
    }
}

static LOG:Mutex<Vec<String>> = Mutex::new(Vec::new());

fn log_add(s: String) {
    let mut log = LOG.lock().unwrap();
    log.push(s);
}

fn get_log() -> Vec<String> {
    let log = LOG.lock().unwrap();
    return log.clone();
}

lazy_static! {
    static ref VARS: Mutex<HashMap<String, CalcNumber>> = Mutex::new(HashMap::new());
}

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

    for line in get_log().iter() {
        fp.write(line.as_bytes())?;
        fp.write("\n".as_bytes())?;
    }
    Ok(())
}

pub fn eval(input: String) -> Result<CalcNumber, ErrorMsg>{
    if !is_parens_correct(input.chars()) { 
        return Err((ERROR_MAP[&WRON_PAREN_ERROR].to_string(), None));
    }
    match lexer(input) {
        Ok(tokens) => {
            log_add(format!("Tokens {:?}", tokens));
            let infixed_tokens = shunting_yard_algorithm(tokens.clone());
            log_add(format!("Infixed syntax {:?}", infixed_tokens));
            let expression_tree = generate_ast(infixed_tokens);
            log_add(format!("Expression tree {:?}", expression_tree));
            match expression_tree {
                Ok(res) => {
                    match res.eval() {
                        Ok(res) => {
                            log_add(format!("Output {:?}", res));
                            log_add("".to_string());
                            Ok(res)
                        },
                        Err((err_code, err_idx)) => {
                            return Err((ERROR_MAP[&err_code].to_string(), err_idx));
                        }
                    }
                },
                Err((err_code, err_idx)) => Err((ERROR_MAP[&err_code].to_string(), err_idx))
            }
        },
        Err(error) => {
            return Err((error, None));
        }
    }
}



fn op_precedence(token: TOKEN) -> u32 {
    match token {
        TOKEN::FACT(_) => 7,
        TOKEN::POW(_) | TOKEN::SQRT(_) => 6,
        TOKEN::DIV(_) | TOKEN::MUL(_) | TOKEN::MOD(_) => 5,
        TOKEN::PLUS(_) | TOKEN::MINUS(_) => 4,
        TOKEN::EQUAL(_) | TOKEN::NOT(_) | TOKEN::NEQUAL(_) |
        TOKEN::GT(_) | TOKEN::GE(_) | TOKEN::LT(_) | TOKEN::LE(_) => 3,
        TOKEN::AND(_) | TOKEN::BAND(_) => 2,
        TOKEN::OR(_) | TOKEN::BOR(_) | TOKEN::XOR(_) | TOKEN::BXOR(_) => 1,
        TOKEN::LPAREN(_) | TOKEN::RPAREN(_) | TOKEN::NUMBER(_, _) | TOKEN::ASSIGN(_) | TOKEN::VAR(_) => 0
    }
}

fn op_associative(token: TOKEN) -> Associativity {
    match token {
        TOKEN::LPAREN(_) | TOKEN::RPAREN(_) | TOKEN::ASSIGN(_) | TOKEN::VAR(_) => Associativity::NOT,
        TOKEN::NUMBER(_, _) => unreachable!(),
        TOKEN::POW(_) | TOKEN::SQRT(_) | TOKEN::NOT(_) => Associativity::RIGHT,
        _ => Associativity::LEFT
    }
}

fn factorial(n: CalcNumber) -> CalcNumber {
    let mut res = Wrapping(1);
    if n.0 == 0 || n.0 == 1 { return res; }
    for i in 1..n.0+1 { res *= i; }
    return res;
}

fn to_bool(n: CalcNumber) -> bool {
    if n.0 > 0 { return true; }
    return false;
}

fn to_calc_num(b: bool) -> CalcNumber {
    if b { return Wrapping(1); }
    return Wrapping(0);
}

fn lexer(input: String) -> Result<Vec<TOKEN>, String> {
    let mut tokens: Vec<TOKEN> = Vec::new();
    let mut i: usize = 0;
    let mut nc: char;
    while i < input.len() {
        nc = input.chars().nth(i).unwrap();
        if nc != ' ' {
            match nc {
                '+' => tokens.push(TOKEN::PLUS(i)),
                '-' => tokens.push(TOKEN::MINUS(i)),
                '*' => {
                    if i+1 < input.len() && input.chars().nth(i+1).unwrap() == '*' {
                        i+=1;
                        tokens.push(TOKEN::POW(i));
                    }else{
                        tokens.push(TOKEN::MUL(i));
                    }
                },
                '(' => tokens.push(TOKEN::LPAREN(i)),
                ')' => tokens.push(TOKEN::RPAREN(i)),
                '/' => tokens.push(TOKEN::DIV(i)),
                '!' => tokens.push(TOKEN::FACT(i)),
                '@' => tokens.push(TOKEN::SQRT(i)),
                '|' => {
                    if i+1 < input.len() && input.chars().nth(i+1).unwrap() == '|' {
                        i+=1;
                        tokens.push(TOKEN::OR(i));
                    }else{
                        tokens.push(TOKEN::BOR(i));
                    }
                },
                '&' => {
                    if i+1 < input.len() && input.chars().nth(i+1).unwrap() == '&' {
                        i+=1;
                        tokens.push(TOKEN::AND(i));
                    }else{
                        tokens.push(TOKEN::BAND(i));
                    }
                },
                '^' => {
                    if i+1 < input.len() && input.chars().nth(i+1).unwrap() == '^' {
                        i+=1;
                        tokens.push(TOKEN::XOR(i));
                    }else{
                        tokens.push(TOKEN::BXOR(i));
                    }
                },
                '%' => tokens.push(TOKEN::MOD(i)),
                '=' => {
                    if i+1 < input.len() && input.chars().nth(i+1).unwrap() == '=' {
                        i+=1;
                        tokens.push(TOKEN::EQUAL(i));
                    }else {
                        tokens.push(TOKEN::ASSIGN(i));
                    }
                },
                '~' => {
                    if i+1 < input.len() && input.chars().nth(i+1).unwrap() == '=' {
                        i+=1;
                        tokens.push(TOKEN::NEQUAL(i));
                    }else{
                        tokens.push(TOKEN::NOT(i));
                    }
                },
                '>' => {
                    if i+1 < input.len() && input.chars().nth(i+1).unwrap() == '=' {
                        i+=1;
                        tokens.push(TOKEN::GE(i));
                    }else{
                        tokens.push(TOKEN::GT(i));
                    }
                },
                '<' => {
                    if i+1 < input.len() && input.chars().nth(i+1).unwrap() == '=' {
                        i+=1;
                        tokens.push(TOKEN::LE(i));
                    }else{
                        tokens.push(TOKEN::LT(i));
                    }
                },
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
                    } else { 
                        return Err(format!("{}: {}", nc, ERROR_MAP[&UNKNOWN_TOKEN_ERROR])); 
                    }
                }
            }
        }
        i+=1;
    }

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
            TOKEN::NUMBER(_, _) => output.push_back(token),
            _ => {
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
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push_back(op);
    }

    return output;
    
}

fn generate_ast(tokens: VecDeque<TOKEN>) -> Result<ASTNode, ErrorCode> {
    let mut stack: Vec<ASTNode> = Vec::new();

    for token in tokens {
        match token {
            // Binary op
            TOKEN::PLUS(char_idx) | TOKEN::MINUS(char_idx) |
            TOKEN::MUL(char_idx) | TOKEN::DIV(char_idx) |
            TOKEN::POW(char_idx) | TOKEN::OR(char_idx) |
            TOKEN::BOR(char_idx) | TOKEN::AND(char_idx) | 
            TOKEN::BAND(char_idx) | TOKEN::XOR(char_idx) |
            TOKEN::BXOR(char_idx) | TOKEN::MOD(char_idx) | 
            TOKEN::EQUAL(char_idx) | TOKEN::NEQUAL(char_idx) |
            TOKEN::GT(char_idx) | TOKEN::GE(char_idx) |
            TOKEN::LT(char_idx) | TOKEN::LE(char_idx) | TOKEN::ASSIGN(char_idx) => {
                let left = Box::new(match stack.pop(){
                    Some(v) => v,
                    None => { return Err((ARG_MISS_ERROR, Some(char_idx))); }
                });
                let right = Box::new(match stack.pop() {
                    Some(v) => v,
                    None => { return Err((ARG_MISS_ERROR, Some(char_idx))); }
                });
                stack.push(ASTNode::BOperator { op: token, left: right, right: left })
            },
            // left unary op
            TOKEN::FACT(char_idx) => {
                match stack.pop(){
                    Some(v) =>  stack.push(ASTNode::ULOperator { op: token, left: Box::new(v) }),
                    None => { return Err((ARG_MISS_ERROR, Some(char_idx))); }
                };
            },
            // right unary op
            TOKEN::SQRT(char_idx) | TOKEN::NOT(char_idx) => {
                match stack.pop() {
                    Some(v) => { stack.push(ASTNode::UROperator { op: token, right: Box::new(v) }) },
                    None => { return Err((ARG_MISS_ERROR, Some(char_idx))); }
                };
            }
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