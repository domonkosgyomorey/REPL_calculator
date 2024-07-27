use std::io::prelude::*;
use std::fmt::Debug;
mod calc;

#[cfg(test)]
mod tests;

enum COMMAND {
    QUIT,
    HELP,
    EMPTY,
    EVAL
}

fn print_help(){
    println!("========= HELP =========");
    println!("commands: \x1b[1;36m(quit, q)\x1b[0m");
    println!("{:widthN$}|{:width$}", " Name", " Command", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Add", "  +", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Sub", "  -", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Mul", "  *", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Div", "  /", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Exp", "  **", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Factorial", "  !", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Square root", "  s", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Mod", "  %", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Equals", "  ==", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Not", "  n", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Not Equal", "  n=", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Logic OR", "  ||", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Bit OR", "  |", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Logic AND", "  &&", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Bit AND", "  &", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Logic XOR", "  ^^", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Bit XOR", "  ^", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Greater Than", "  >", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Greater Equal", "  >=", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Less Than", "  <", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Less Equal", "  <=", widthN=15, width=5);
    println!("evaluation:");
    println!("\x1b[1;36m\t3*1-3+2 n= s16/2!-(6&3) && s(2**3*50)>=19\x1b[0m");
    println!("\x1b[1;32m\t=> 1\x1b[0m")
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

fn print_error<T: Debug>(msg: T, char_idx: Option<usize>) {
    let maybe_char_idx: String = match char_idx {
        Some(ci) => format!(" at {}", ci),
        None => "".to_string(),
    };
    println!("\x1b[1;31mError: {:?}{}\x1b[0m", msg, maybe_char_idx);
}

fn print_result<T: Debug>(msg: T) {
    println!("\x1b[1;32m=> {:?}\x1b[0m", msg);
}

fn main() -> std::io::Result<()>{
    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();
        let input: String = get_line().trim().to_string();
        match get_command(&input) {
            COMMAND::EVAL => {
                match calc::eval(input) {
                    Ok(res) => print_result(res),
                    Err((err, char_idx)) => print_error(err, char_idx)
                };
            },
            COMMAND::HELP => print_help(),
            COMMAND::EMPTY => { continue; }
            COMMAND::QUIT => { break; }
        }

        calc::write_log("log.txt")?;
    }
    Ok(())
}
