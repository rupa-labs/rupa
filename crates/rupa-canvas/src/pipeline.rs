use crate::api::Command;

/// Integration logic for custom canvas draw calls.
pub struct Pipeline {
    queue: Vec<Command>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn push(&mut self, command: Command) {
        self.queue.push(command);
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn commands(&self) -> &[Command] {
        &self.queue
    }
}
