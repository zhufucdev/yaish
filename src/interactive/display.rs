use std::io::Write;
use std::ops::Deref;
use console::{Key, Term};
use crate::config::completion::{Command, Completion, Suggestion};
use crate::interactive::behavior::{CliBehavior, Focus, InteractiveBehavior};


pub struct Display {
    pub term: Term,
    focus: Focus,
    behavior: Box<dyn InteractiveBehavior>,
}

impl From<Term> for Display {
    fn from(term: Term) -> Display {
        return Display {
            term,
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
            match self.term.read_key() {
                Ok(key) => {
                    let current_behavior = self.behavior.as_mut();
                    match key {
                        Key::Char(c) => {
                            current_behavior.on_char_received(c, &mut self.term);
                            if self.behavior.should_execute() {
                                break;
                            }
                        }

                        others => {
                            current_behavior.on_key_pressed(others, &mut self.term);
                            if self.behavior.should_execute() {
                                break;
                            }
                        }
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
            self.term.write("🤔 ".as_bytes()).unwrap();
            self.term.flush().unwrap();

            self.inline_loop();
            self.behavior.execute();
            println!()
        }
    }
}