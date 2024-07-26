use std::io::prelude::*;
mod calc;

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

fn main() -> std::io::Result<()>{
    let mut output: Vec<String> = Vec::new();
    loop {

        print!("$ ");
        std::io::stdout().flush().unwrap();
        let input: String = get_line().trim().to_string();
        match get_command(&input) {
            COMMAND::EVAL => calc::eval(input, &mut output),
            COMMAND::HELP => print_help(),
            COMMAND::EMPTY => { continue; }
            COMMAND::QUIT => { break; }
        }

        for line in output.iter() {
            println!("{}", line);
        }
        output.clear();

        calc::write_log("log.txt")?;
    }
    Ok(())
}
