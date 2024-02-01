use std::io;
use std::process::Command;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let command = input.trim();

    Command::new(command).spawn().unwrap();
}
