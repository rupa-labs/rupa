use crate::Component;

/// A utility to manage parent-child relationships in a standardized way.
pub struct Children {
    pub list: Vec<Box<dyn Component>>,
}

impl Children {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add(&mut self, child: Box<dyn Component>) {
        self.list.push(child);
    }

    pub fn append(&mut self, mut children: Vec<Box<dyn Component>>) {
        self.list.append(&mut children);
    }

    pub fn render_all(&self) {
        for child in &self.list {
            child.render();
        }
    }
}
