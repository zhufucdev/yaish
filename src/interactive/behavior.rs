use std::io::Write;
use console::{Key, Term};
use crate::config::behavior;
use crate::config::completion::{Command, Completion, Suggestion};
use crate::interactive::display::Display;

pub enum Focus {
    CLI,
    COMPLETIONS,
}

pub trait InteractiveBehavior {
    fn on_key_pressed(&mut self, key: Key, term: &mut Term);
    fn on_char_received(&mut self, c: char, term: &mut Term);
    fn should_execute(&self) -> bool;
    fn execute(&self);
}

pub struct AutoCompletionBehavior {
    current_suggestions: Vec<Suggestion>,
    columns: u16,
    col_width: u16,
}

impl AutoCompletionBehavior {
    pub fn new(suggestions: Vec<Suggestion>, term: Term) -> AutoCompletionBehavior {
        let columns: u16 = suggestions.len().min(4) as u16;
        let width = term.size().0 / columns;

        return AutoCompletionBehavior {
            columns,
            col_width: width,
            current_suggestions: vec![],
        };
    }
}

impl InteractiveBehavior for AutoCompletionBehavior {
    fn on_key_pressed(&mut self, key: Key, term: &mut Term) {}

    fn on_char_received(&mut self, c: char, term: &mut Term) {}

    fn should_execute(&self) -> bool {
        return false;
    }
    fn execute(&self) {}
}

pub struct CliBehavior {
    buffer: String,
    execute: bool,
}

impl CliBehavior {
    pub fn new() -> CliBehavior {
        return CliBehavior {
            buffer: String::new(),
            execute: false,
        };
    }

    fn parse_arguments(&self) -> Vec<String> {
        let mut arguments: Vec<String> = Vec::new();
        let mut trapper: char = ' ';
        let mut escaped = false;
        let mut current = String::new();

        for entry in self.buffer.chars() {
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
}

impl InteractiveBehavior for CliBehavior {
    fn should_execute(&self) -> bool {
        return self.execute;
    }

    fn execute(&self) {
        let args = self.parse_arguments();
        let command = Command::from(args);

        println!();
        println!("Command: {}", command.get_command());
        println!("Args: {:?}", command.get_arguments());
    }

    fn on_key_pressed(&mut self, key: Key, term: &mut Term) {
        match key {
            Key::Backspace => {
                term.clear_chars(1).unwrap();
                self.buffer.pop();
            }
            Key::Enter => {
                self.execute = true;
            }
            _ => {}
        }
    }

    fn on_char_received(&mut self, c: char, term: &mut Term) {
        let mut buf = [0; 8];
        c.encode_utf8(&mut buf);
        term.write(&buf).unwrap();
        term.flush().unwrap();

        self.buffer.push(c);
    }
}