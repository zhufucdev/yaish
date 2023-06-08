use std::io::{Stdout, Write};
use crossterm::{cursor, QueueableCommand};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::ExecutableCommand;
use crossterm::terminal::{Clear, ClearType};
use crate::config::behavior;
use crate::config::completion::{Command, Completion, Suggestion};
use crate::interactive::display::Display;

pub enum Focus {
    CLI,
    COMPLETIONS,
}

pub trait InteractiveBehavior {
    fn on_key_pressed(&mut self, ev: KeyEvent, out: &mut Stdout);
    fn on_char_received(&mut self, c: char, ev: KeyEvent, out: &mut Stdout);
    fn should_execute(&self) -> bool;
    fn execute(&self);
}

pub struct AutoCompletionBehavior {
    current_suggestions: Vec<Suggestion>,
    columns: u16,
    col_width: u16,
}

impl AutoCompletionBehavior {
    pub fn new(suggestions: Vec<Suggestion>, width: u16) -> AutoCompletionBehavior {
        let columns: u16 = suggestions.len().min(4) as u16;
        let width = width / columns;

        return AutoCompletionBehavior {
            columns,
            col_width: width,
            current_suggestions: vec![],
        };
    }
}

impl InteractiveBehavior for AutoCompletionBehavior {
    fn on_key_pressed(&mut self, ev: KeyEvent, out: &mut Stdout) {}

    fn on_char_received(&mut self, c: char, ev: KeyEvent, out: &mut Stdout) {}

    fn should_execute(&self) -> bool {
        return false;
    }
    fn execute(&self) {}
}

pub struct CliBehavior {
    buffer: String,
    execute: bool,
    cursor: u16,
}

impl CliBehavior {
    pub fn new() -> CliBehavior {
        return CliBehavior {
            buffer: String::new(),
            execute: false,
            cursor: 0,
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
    fn on_key_pressed(&mut self, ev: KeyEvent, out: &mut Stdout) {
        match ev.code {
            KeyCode::Backspace => {
                out.queue(cursor::MoveLeft(1)).unwrap();
                out.queue(Clear(ClearType::FromCursorDown)).unwrap();
                out.flush().unwrap();

                self.buffer.remove(self.buffer.len() - self.cursor as usize - 1);
                out.write_all(
                    self.buffer.get(self.buffer.len() - self.cursor as usize..self.buffer.len())
                        .unwrap().as_bytes()
                ).unwrap();
                out.flush().unwrap();

                if self.cursor > 0 {
                    out.execute(cursor::MoveLeft(self.cursor)).unwrap();
                }
            }
            KeyCode::Left => {
                out.execute(cursor::MoveLeft(1)).unwrap();
                self.cursor += 1;
            }
            KeyCode::Right => {
                out.execute(cursor::MoveRight(1)).unwrap();
                self.cursor = (self.cursor - 1).max(0);
            }
            KeyCode::Enter => {
                self.execute = true;
            }
            _ => {}
        }
    }

    fn on_char_received(&mut self, c: char, ev: KeyEvent, out: &mut Stdout) {
        if ev.modifiers == KeyModifiers::CONTROL && c == 'd' {
            Display::quit();
        }


        out.execute(Clear(ClearType::FromCursorDown)).unwrap();
        self.buffer.insert(self.buffer.len() - self.cursor as usize, c);

        out.write_all(
            self.buffer.get(self.buffer.len() - 1 - self.cursor as usize..self.buffer.len())
                .unwrap().as_bytes()
        ).unwrap();
        out.flush().unwrap();

        if self.cursor > 0 {
            out.execute(cursor::MoveLeft(self.cursor)).unwrap();
        }
    }

    fn should_execute(&self) -> bool {
        return self.execute;
    }

    fn execute(&self) {
        let args = self.parse_arguments();
        let command = Command::from(args);

        println!("Command: {}", command.get_command());
        println!("Args: {:?}", command.get_arguments());
    }
}