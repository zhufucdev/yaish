use std::io::{Write};

use console::{Term, Key};

fn main() {
    let mut term = Term::stdout();
    if !term.is_term() {
        println!("Liar! This is not a terminal!")
    }

    loop {
        term.write("🤔 ".as_bytes()).unwrap();
        term.flush().unwrap();

        loop {
            match term.read_key() {
                Ok(key) => {
                    match key {
                        Key::Char(c) => {
                            let mut buf = [0; 8];
                            c.encode_utf8(&mut buf);
                            term.write(&buf).unwrap();
                            term.flush().unwrap();
                        }

                        Key::Enter => { break; }

                        Key::Backspace => {
                            term.clear_chars(1).unwrap();
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

        println!();
    }
}
