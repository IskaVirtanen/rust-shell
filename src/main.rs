use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child,Command,Stdio};
use std::env;
fn main() {
    loop{
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next(){

            // everything after the first whitespace char
            // is interpreted as args to the command
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap_or("");
            let _args = parts;

		        match command {
                "cd" => {
                    // default to '/' as new directory if one was not provided
                    let new_dir = _args.peekable().peek().map_or("/",|x| *x);
                    let root = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&root){
                        eprintln!("{}",e);
                    }
                    previous_command = None;
                },
                "exit" => return,
                command => {
                   let stdin = previous_command.map_or(
                            std::process::Stdio::inherit(),
                                |output: Child|{ Stdio::from(output.stdout.unwrap())

                                });
                   let _stdout = if commands.peek().is_some(){
                       Stdio::piped()
                   }else{
                        Stdio::inherit()
                   };

                   let _output = Command::new(command)
                       .args(_args)
                       .stdin(stdin)
                       .stdout(_stdout)
                       .spawn();
                    match _output{
                        Ok(output) =>{
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}",e);
                        }
                   }; 
                }
            }
        }
        if let Some(mut final_command) = previous_command{
            final_command.wait().unwrap();
        }
    }
}

