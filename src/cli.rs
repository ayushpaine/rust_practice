use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Ayush";

    if command == "hello" {
        println!("Hi {} joote kitne ke h?", name);
    }
}
