use rupa_core::{Component, VNode, ViewCore, Id, Signal};
use std::sync::Arc;

/// A reactive progress bar component for the terminal.
#[derive(Clone)]
pub struct Progress {
    pub id: String,
    pub label: Signal<String>,
    pub value: Signal<f32>, // 0.0 to 1.0
    pub view: Arc<ViewCore>,
}

impl Progress {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: Id::next().to_string(),
            label: Signal::new(label.into()),
            value: Signal::new(0.0),
            view: Arc::new(ViewCore::new()),
        }
    }

    pub fn set_value(&self, val: f32) {
        self.value.set(val.clamp(0.0, 1.0));
    }

    pub fn set_label(&self, label: impl Into<String>) {
        self.label.set(label.into());
    }
}

impl Component for Progress {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }

    fn render(&self) -> VNode {
        let label = self.label.get();
        let value = self.value.get();
        let filled = (value * 20.0) as usize;
        let empty = 20 - filled;
        
        let bar = format!("[{}{}] {:.1}%", 
            "█".repeat(filled), 
            "░".repeat(empty), 
            value * 100.0
        );

        VNode::element("div")
            .with_style(self.view.style().clone())
            .with_child(VNode::text(format!("{}: {}", label, bar)))
    }
}
