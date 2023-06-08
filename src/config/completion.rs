use std::ops::Add;
use crossterm::style::Stylize;

pub struct Command {
    commands: Vec<String>,
    command: String,
    arguments: Vec<String>,
}

impl From<Vec<String>> for Command {
    fn from(arguments: Vec<String>) -> Command {
        if arguments.is_empty() {
            panic!("Empty arguments")
        }

        let owned = arguments.clone();

        return Command {
            commands: arguments,
            command: String::from(owned.get(0).unwrap()),
            arguments: owned[1..].to_owned(),
        };
    }
}

impl ToString for Command {
    fn to_string(&self) -> String {
        self.commands.join(" ")
    }
}

impl Command {
    pub fn get_command(&self) -> &str {
        return self.command.as_str();
    }

    pub fn get_arguments(&self) -> Vec<String> {
        return self.arguments.clone();
    }
}

pub struct Suggestion {
    pub tooltip: String
}

pub struct Completion {
    pub tooltip: String
}
