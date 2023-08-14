//this tool will be use to made request to different LLM API
use std::io;

fn main() {
    let mut cmd: String = String::new();

    while cmd != "exit" {
        cmd = "".to_string();
        println!("Insert a command:");
        cmd = read_input(cmd);
        println!("You typed: {}", cmd);
    }
}
//this function will read the input from the user
fn read_input(mut input: String ) -> String {
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}