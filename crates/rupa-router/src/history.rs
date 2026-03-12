use rupa_signals::Signal;

/// The primary Port for managing navigation history.
pub trait History: Send + Sync {
    fn push(&self, path: &str);
    fn replace(&self, path: &str);
    fn back(&self);
    fn current(&self) -> String;
}

/// An in-memory implementation of the History Port.
pub struct MemoryHistory {
    pub stack: Signal<Vec<String>>,
    pub index: Signal<usize>,
}

impl MemoryHistory {
    pub fn new(initial: impl Into<String>) -> Self {
        Self {
            stack: Signal::new(vec![initial.into()]),
            index: Signal::new(0),
        }
    }
}

impl History for MemoryHistory {
    fn push(&self, path: &str) {
        self.stack.update(|s| {
            let idx = self.index.get();
            s.truncate(idx + 1);
            s.push(path.to_string());
        });
        self.index.set(self.stack.get().len() - 1);
    }

    fn replace(&self, path: &str) {
        self.stack.update(|s| {
            let idx = self.index.get();
            s[idx] = path.to_string();
        });
    }

    fn back(&self) {
        let idx = self.index.get();
        if idx > 0 {
            self.index.set(idx - 1);
        }
    }

    fn current(&self) -> String {
        let s = self.stack.get();
        s[self.index.get()].clone()
    }
}
