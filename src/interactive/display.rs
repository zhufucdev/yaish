use std::io::{Stdout, Write};
use std::process::exit;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::terminal;
use crate::config::completion::{Completion, Suggestion};
use crate::interactive::behavior::{CliBehavior, Focus, InteractiveBehavior};


pub struct Display {
    pub stdout: Stdout,
    focus: Focus,
    behavior: Box<dyn InteractiveBehavior>,
}

impl From<Stdout> for Display {
    fn from(stdout: Stdout) -> Display {
        return Display {
            stdout,
            focus: Focus::CLI,
            behavior: Box::new(CliBehavior::new()),
        };
    }
}


impl Display {
    fn set_suggestions(&mut self, suggestions: Vec<Suggestion>) {}

    fn show_completion(&mut self, completion: Completion) {}

    fn inline_loop(&mut self) {
        self.behavior = Box::new(CliBehavior::new());
        self.focus = Focus::CLI;

        loop {
            match read() {
                Ok(event) => {
                    match event {
                        Event::Key(ev) => {
                            let current_behavior = self.behavior.as_mut();
                            match ev.code {
                                KeyCode::Char(c) => {
                                    current_behavior.on_char_received(c, ev, &mut self.stdout);
                                    if self.behavior.should_execute() {
                                        break;
                                    }
                                }

                                _ => {
                                    current_behavior.on_key_pressed(ev, &mut self.stdout);
                                    if self.behavior.should_execute() {
                                        break;
                                    }
                                }
                            }
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
    }

    pub fn main_loop(&mut self) {
        loop {
            terminal::enable_raw_mode().unwrap();
            self.stdout.write("🤔 ".as_bytes()).unwrap();
            self.stdout.flush().unwrap();

            self.inline_loop();
            terminal::disable_raw_mode().unwrap();

            println!();
            self.behavior.execute();
            println!();
        }
    }

    pub fn quit() {
        terminal::disable_raw_mode().unwrap();
        exit(0);
    }
}