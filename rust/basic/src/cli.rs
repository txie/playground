use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = args[2].clone();

    println!("Args: {:?}", args);
    println!("Command: {}", command);
    if command == "hello" {
        println!("Hi {}, how are you", name);
    }
    else {
        println!("It's not a valid command");
    }
}