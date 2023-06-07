use std::io::{Write};
use console::{Term, Key};
use crate::config::behavior;


pub fn main_loop(mut term: Term) {
    loop {
        term.write("🤔 ".as_bytes()).unwrap();
        term.flush().unwrap();

        let mut arguments: Vec<String> = Vec::new();
        inline_loop(&mut term, &mut arguments);

        println!("{:?}", arguments);
    }
}

fn inline_loop(term: &mut Term, arguments: &mut Vec<String>) {
    loop {
        let mut current = String::new();
        let mut execute = false;
        let mut trapper: char = ' ';

        loop {
            match term.read_key() {
                Ok(key) => {
                    match key {
                        Key::Char(c) => {
                            let mut buf = [0; 8];
                            c.encode_utf8(&mut buf);
                            term.write(&buf).unwrap();
                            term.flush().unwrap();

                            if c == ' ' && trapper == ' ' && !current.is_empty() {
                                arguments.push(current);
                                break;
                            } else if behavior::TRAP_CHARS.contains(c) {
                                if c == trapper {
                                    // escape
                                    trapper = ' ';
                                } else if trapper == ' ' {
                                    trapper = c;
                                } else {
                                    current.push(c);
                                }
                            } else {
                                current.push(c);
                            }
                        }

                        Key::Enter => {
                            arguments.push(current);
                            execute = true;
                            break;
                        }

                        Key::Backspace => {
                            term.clear_chars(1).unwrap();
                            current.pop();
                        }

                        _ => {}
                    }
                }

                Err(_) => {
                    println!();
                    println!("Fuck that. Everything goes wrong. I am not interactive anymore.");
                    return;
                }
            }
        }

        if execute {
            break;
        }
    }
}