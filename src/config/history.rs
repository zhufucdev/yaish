use crate::config::completion::Command;

pub(crate) struct History {
    commands: Vec<Command>,
    index: i32
}

impl History {
    pub(crate) fn new() -> History {
        History {
            commands: vec![],
            index: 0
        }
    }

    pub(crate) fn push(&mut self, historical: Command) {
        self.commands.push(historical);
        self.index = self.commands.len() as i32;
    }

    pub(crate) fn wind(&mut self) -> Option<&Command> {
        if self.index < 0 {
            return None;
        }
        self.index -= 1;
        self.commands.get(self.index as usize)
    }

    pub(crate) fn unwind(&mut self) -> Option<&Command> {
        if self.index >= self.commands.len() as i32 {
            return None;
        }

        self.index += 1;
        self.commands.get(self.index as usize)
    }

    pub(crate) fn present(&self) -> Option<&Command> {
        self.commands.get(self.index as usize)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub(crate) fn reset(&mut self) {
        self.index = self.commands.len() as i32;
    }
}