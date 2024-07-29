use std::io::prelude::*;
use colored::*;
mod calc;


#[cfg(test)]
mod tests;

enum COMMAND {
    QUIT,
    HELP,
    EMPTY,
    VARS,
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
    println!("{:widthN$}|{:width$}", " Square root", "  @", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Mod", "  %", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Equals", "  ==", widthN=15, width=5);
    println!("{:widthN$}|{:width$}", " Not", "  ~", widthN=15, width=5);
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
    println!("{}", "\t3*1-3+2 ~= @16/2!-(6&3) && @(2**3*50)>=19".green());
    println!("{}", "\t=> 1".green())
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
    if cmd=="vars" { return COMMAND::VARS; }
    if cmd=="" { return COMMAND::EMPTY; }
    return COMMAND::EVAL;
}

fn print_error(msg: String, char_idx: Option<usize>) {
    let maybe_char_idx: String = match char_idx {
        Some(ci) => format!(" at {}", ci),
        None => "".to_string(),
    };
    println!("{} {}{}", "Error:".red().bold(), msg.red(), maybe_char_idx.red());
}

fn print_result(msg: String) {
    println!("{} {}", "=>".green(), msg.green().bold());
}

fn main() -> std::io::Result<()>{
    loop {
        print!("$ ");
        std::io::stdout().flush().unwrap();
        let input: String = get_line().trim().to_string();
        match get_command(&input) {
            COMMAND::EVAL => {
                match calc::eval(input) {
                    Ok(res) => print_result(res.to_string()),
                    Err((err, char_idx)) => print_error(err, char_idx)
                };
            },
            COMMAND::HELP => print_help(),
            COMMAND::VARS => calc::get_vars().into_iter().for_each(|line| println!("{}", line.blue().bold())),
            COMMAND::EMPTY => { continue; }
            COMMAND::QUIT => { break; }
        }

        calc::write_log("log.txt")?;
    }
    Ok(())
}
