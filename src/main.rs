use std::io::Write;
use std::process::Command;
use std::env;
use std::path::Path;
fn main() {
    loop{
        print!("> ");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // everything after the first whitespace char
        // is interpreted as args to the command
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let _args = parts;

        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = _args.peekable().peek().map_or("/",|x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root){
                    eprintln!("{}",e);
                }
            },
            "exit" => return,
            command => {
                let child = Command::new(command).args(_args).spawn(); 

                match child{
                    Ok(mut child) => {let _ = child.wait();},
                    Err(e) => eprintln!("{}",e),
                };
            }
        }
    }
}

