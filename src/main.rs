use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    loop {
        print!("> "); // prompt
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                // goes to "/" by default
                let target = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(target);

                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            command => {
                let mut child = Command::new(command).args(args).spawn().unwrap();
                child.wait().expect("Failed to wait for child process"); // wait for commands to be done, like a queue
            }
        }
    }
}
