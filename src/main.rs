use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

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
                                // at some point for languages just say the extension again... or support idk
                                "jpeg" | "jpg" => "JPEG image",
                                "png" => "PNG image",
                                "gif" => "GIF animation",
                                "txt" => "Text file",
                                "py" => "Python file",
                                "rs" => "Rust file",
                                "md" => "Markdown file",
                                // maybe find a way to differentiate?
                                _ => "Binary executable or unsupported",
                            };
                            println!("File type: {}", filetype)
                        }
                    } else {
                        println!("{} does not exist", file.display());
                    }
                }
                "sysinfo" => {
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    let s = System::new_with_specifics(
                        RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
                    );
                    println!("OS: {}", System::name().unwrap());
                    println!("CPU: {}", s.cpus()[0].brand());
                    println!("MEM: {}", sys.total_memory()); // TODO: convert to GB
                }
                "wordcount" => {
                    let target = args.peekable().peek().map_or("", |x| *x);
                    let file = Path::new(target);
                    if file.exists() {
                        if file.is_dir() {
                            println!("{} is a directory", file.display());
                        } else {
                            todo!();
                        }
                    } else {
                        println!("{} does not exist", file.display());
                    }
                }
                "filesize" => {
                    todo!();
                }
                "calc" => {
                    // need to parse any amount of args?
                    todo!();
                }
                "rand" => {
                    todo!();
                }
                "datetime" => {
                    todo!();
                }
                "repeat" => {
                    let amount: &str = args.peekable().peek().map_or("", |x| *x);
                    // take in command and args :TODO
                    if let Ok(result) = amount.parse::<i32>() {
                        println!("{}", result);
                        todo!();
                    }
                }
                "note" => {
                    // and all the other shownotes and stuff
                    todo!();
                }
                "search" => {
                    // IDK if i want to implement this myself. May just grep.
                    todo!();
                }
                "rename" => {
                    todo!();
                }
                "copy" => {
                    todo!();
                }
                "move" => {
                    todo!();
                }
                "create" => {
                    todo!();
                }
                "delete" => {
                    // make safer
                    todo!();
                }
                "permissions" => {
                    todo!();
                }
                "fileinfo" => {
                    todo!();
                }
                "preview" => {
                    let target = args.peekable().peek().map_or("", |x| *x);
                    let file = Path::new(target);
                    if file.exists() {
                        if file.is_dir() {
                            println!("{} is a directory", file.display());
                        } else {
                            let contents = fs::read_to_string(file).expect("Unable to read file");
                            println!("Contents: \n\n{}", contents)
                        }
                    } else {
                        println!("{} does not exist", file.display());
                    }
                }
                "alias" => {
                    todo!();
                }
                "setenv" => {
                    todo!();
                }
                "help" => {
                    todo!();
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
