use std::io::Write;


fn main() {
    loop{
        print!("> ");
        let _ = std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        let mut child = std::process::Command::new(command).spawn().unwrap();

        let _ = child.wait();
        
    }
}
