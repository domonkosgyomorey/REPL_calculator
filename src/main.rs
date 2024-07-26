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
