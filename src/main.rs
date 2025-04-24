use std::io::Write;


fn main() {
    loop{
        print!("> ");
        let _ = std::io::stdout().flush();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        // everything after the first whitespace char
        // is interpreted as args to the command
        let parts = input.trim().split_whitespace();
        let command = input.trim();
        let _args = parts;
        let mut child = std::process::Command::new(command).spawn().unwrap();

        let _ = child.wait();
        
    }
}
