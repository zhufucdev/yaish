use std::io::{Stdout, Write};
use crossterm::{cursor, QueueableCommand};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::ExecutableCommand;
use crossterm::style::Stylize;
use crossterm::terminal::{Clear, ClearType};
use crate::config::behavior;
use crate::config::completion::{Command, Completion, Suggestion};
use crate::config::history::History;
use crate::interactive::display::{Beep, Display};

pub enum Focus {
    CLI,
    COMPLETIONS,
}

pub trait InteractiveBehavior {
    fn on_key_pressed(&mut self, ev: KeyEvent, out: &mut Stdout);
    fn on_char_received(&mut self, c: char, ev: KeyEvent, out: &mut Stdout);
    fn should_execute(&self) -> bool;
    fn execute(&mut self);
    fn reset(&mut self);
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
    fn execute(&mut self) {}
    fn reset(&mut self) {}
}

pub struct CliBehavior {
    buffer: String,
    execute: bool,
    cursor: u16,
    history: History,
    is_winding: bool,
}

impl CliBehavior {
    pub fn new() -> CliBehavior {
        return CliBehavior {
            buffer: String::new(),
            execute: false,
            cursor: 0,
            history: History::new(),
            is_winding: false,
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

    fn swipe(str: &str, out: &mut Stdout) {
        out.queue(cursor::MoveToColumn(behavior::STARTER.chars().count() as u16 + 1)).ok();
        out.queue(Clear(ClearType::FromCursorDown)).ok();
        out.write(str.as_bytes()).ok();
        out.flush().unwrap();
    }
}

impl InteractiveBehavior for CliBehavior {
    fn on_key_pressed(&mut self, ev: KeyEvent, out: &mut Stdout) {
        // handle history winding
        match ev.code {
            KeyCode::Up => {
                match self.history.wind() {
                    Some(command) => {
                        self.is_winding = true;
                        CliBehavior::swipe(command.to_string().as_str(), out);
                    }

                    None => {
                        if !self.history.is_empty() {
                            CliBehavior::swipe(self.history.unwind().unwrap().to_string().negative().to_string().as_str(), out);
                            self.history.wind();
                        }
                        out.beep();
                    }
                }
            }

            KeyCode::Down => {
                if self.is_winding {
                    match self.history.unwind() {
                        Some(command) => {
                            CliBehavior::swipe(command.to_string().as_str(), out);
                        }

                        None => {
                            self.is_winding = false;
                            CliBehavior::swipe(self.buffer.as_str(), out);
                        }
                    }
                } else {
                    out.beep()
                }
            }

            _ => {
                if self.is_winding {
                    self.is_winding = false;
                    self.buffer = self.history.present().or(Some(&Command::empty())).unwrap().to_string();
                    CliBehavior::swipe(self.buffer.as_str(), out);
                    self.cursor = 0;
                }
            }
        }

        match ev.code {
            KeyCode::Backspace => {
                if self.buffer.len() > 0 {
                    out.queue(cursor::MoveLeft(1)).unwrap();
                    out.queue(Clear(ClearType::FromCursorDown)).unwrap();
                    out.flush().ok();

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
            }
            KeyCode::Left => {
                if self.cursor < self.buffer.len() as u16 {
                    out.execute(cursor::MoveLeft(1)).unwrap();
                    self.cursor += 1;
                }
            }
            KeyCode::Right => {
                if self.cursor > 0 {
                    out.execute(cursor::MoveRight(1)).unwrap();
                    self.cursor -= 1;
                }
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

        // make sure currently it's not winding
        if self.is_winding {
            match self.history.present() {
                Some(command) => {
                    self.buffer = command.to_string();
                    CliBehavior::swipe(self.buffer.as_str(), out);
                }

                None => {
                    // clear because, you know, nothing useful has been found
                    self.buffer = String::new();
                    CliBehavior::swipe("", out);
                }
            }
            self.is_winding = false;
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

    fn execute(&mut self) {
        let args = self.parse_arguments();
        let command = Command::from(args);

        println!("Command: {}", command.get_command());
        println!("Args: {:?}", command.get_arguments());

        if !command.is_empty() {
            self.history.push(command);
        }
    }

    fn reset(&mut self) {
        self.execute = false;
        self.is_winding = false;
        self.buffer = String::new();
    }
}