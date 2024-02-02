use std::env;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main() {
    loop {
        print!("> "); // prompt
        io::stdout().flush();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let mut commands = input.trim().split(" | ").peekable();
        let mut prev_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
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
                "filetype" => {
                    let target = args.peekable().peek().map_or("", |x| *x);
                    let file = Path::new(target);

                    if file.exists() {
                        if file.is_dir() {
                            println!("{} is a directory", file.display());
                        } else {
                            let extension: &str =
                                file.to_str().and_then(|s| s.split('.').last()).unwrap();
                            let filetype = match extension.to_lowercase().as_str() {
                                "jpeg" | "jpg" => "JPEG image",
                                "png" => "PNG image",
                                "gif" => "GIF animation",
                                "txt" => "Text file",
                                "py" => "Python file",
                                "rs" => "Rust file",
                                "md" => "Markdown file",
                                _ => "Binary executable or unsupported",
                            };
                            println!("File type: {}", filetype)
                        }
                    } else {
                        println!("{} does not exist", file.display());
                    }
                }
                "exit" => return,
                command => {
                    let stdin = prev_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        // check if another command is piped behind

                        Stdio::piped()
                    } else {
                        // no more pipes
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            prev_command = Some(output);
                        }
                        Err(e) => {
                            prev_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }
        if let Some(mut final_command) = prev_command {
            // block until the last command is done
            final_command.wait();
        }
    }
}
