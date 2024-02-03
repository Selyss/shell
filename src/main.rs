use chrono::{DateTime, Local, TimeZone, Utc};
use std::env;
use std::fs;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

mod commands;
use crate::commands::filesize;
use crate::commands::filetype;
use crate::commands::wordcount;

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
                    let file: &Path = Path::new(target);

                    if file.exists() {
                        if file.is_symlink() {
                            println!("{} is a symlink", file.display());
                        }
                        if file.is_dir() {
                            println!("{} is a directory", file.display());
                        } else {
                            let filetype = filetype::file_extension(file);
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
                    let path = Path::new(target);
                    if path.exists() {
                        if path.is_dir() {
                            println!("{} is a directory", path.display());
                        } else {
                            wordcount::wordcount(path.to_str().unwrap()); // HACK: ?
                        }
                    } else {
                        println!("{} does not exist", path.display());
                    }
                }
                "filesize" => {
                    let target = args.peekable().peek().map_or("", |x| *x);
                    let path = Path::new(target);
                    if path.exists() {
                        if path.is_dir() {
                            println!("{} is a directory", path.display());
                        } else {
                            filesize::filesize(path.to_str().unwrap()); // HACK: ?
                        }
                    } else {
                        println!("{} does not exist", path.display());
                    }
                }
                "calc" => {
                    // need to parse any amount of args?
                    todo!();
                }
                "rand" => {
                    todo!();
                }
                "datetime" => {
                    let utc_time: DateTime<Utc> = Utc::now();
                    let local_time: DateTime<Local> = utc_time.with_timezone(&Local);

                    println!("UTC time: {}", utc_time);
                    println!("Local time: {}", local_time);
                }
                "repeat" => {
                    let amount: &str = args.peekable().peek().map_or("", |x| *x);
                    // take in command and args :TODO
                    if let Ok(result) = amount.parse::<i32>() {
                        println!("{}", result);
                    }
                }
                "note" => {
                    // and all the other shownotes and stuff
                    todo!();
                }
                "create" => {
                    let target = args.peekable().peek().map_or("", |x| *x);
                    let file = Path::new(target);
                    if file.exists() {
                        if file.is_dir() {
                            println!("{} is a directory", file.display());
                        } else {
                        }
                    } else {
                        println!("{} does not exist", file.display());
                    }
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
