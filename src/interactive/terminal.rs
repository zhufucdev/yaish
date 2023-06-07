use std::io::{Write};
use console::{Term, Key};
use crate::config::{behavior, completion};

pub fn main_loop(mut term: Term) {
    loop {
        term.write("🤔 ".as_bytes()).unwrap();
        term.flush().unwrap();

        let arguments = inline_loop(&mut term);
        let command = completion::Command::from(arguments);

        println!();
        println!("command: {}", command.get_command());
        println!("args: {:?}", command.get_arguments());
    }
}

fn inline_loop(term: &mut Term) -> Vec<String> {
    let mut buffer = String::new();

    loop {
        let execute: bool;

        loop {
            match term.read_key() {
                Ok(key) => {
                    match key {
                        Key::Char(c) => {
                            let mut buf = [0; 8];
                            c.encode_utf8(&mut buf);
                            term.write(&buf).unwrap();
                            term.flush().unwrap();

                            buffer.push(c);
                        }

                        Key::Enter => {
                            execute = true;
                            break;
                        }

                        Key::Backspace => {
                            term.clear_chars(1).unwrap();
                            buffer.pop();
                        }

                        _ => {}
                    }
                }

                Err(_) => {
                    println!();
                    println!("Fuck that. Everything goes wrong. I am not interactive anymore.");
                    return Vec::new();
                }
            }
        }

        if execute {
            break;
        }
    }

    return parse_arguments(buffer);
}

fn parse_arguments(buffer: String) -> Vec<String> {
    let mut arguments: Vec<String> = Vec::new();
    let mut trapper: char = ' ';
    let mut  escaped = false;
    let mut current = String::new();

    for entry in buffer.chars() {
        if !escaped {
            if behavior::TRAP_CHARS.contains(entry) {
                if trapper == entry {
                    // escape
                    trapper = ' ';
                    continue;
                } else if trapper == ' ' {
                    trapper = entry;
                    continue;
                }
            } else if entry == behavior::ESCAPE_CHAR {
                escaped = true;
                continue;
            } else if entry == ' ' && trapper == ' ' && !current.is_empty() {
                arguments.push(current);
                current = String::new();
                continue;
            }
        }

        current.push(entry);
        escaped = false;
    }

    arguments.push(current);

    return arguments;
}