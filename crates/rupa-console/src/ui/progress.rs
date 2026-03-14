use rupa_core::{Component, VNode, Id, Signal, Style};
use std::sync::{Arc, RwLock};

/// A reactive progress bar component for the terminal.
#[derive(Clone)]
pub struct Progress {
    pub id: String,
    pub label: Signal<String>,
    pub value: Signal<f32>, // 0.0 to 1.0
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Progress {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: Id::next().to_string(),
            label: Signal::new(label.into()),
            value: Signal::new(0.0),
            style: Arc::new(RwLock::new(Style::default())),
            prev_vnode: Arc::new(RwLock::new(None)),
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
    fn style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }

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
            .with_style(self.style().read().unwrap().clone())
            .with_child(VNode::text(format!("{}: {}", label, bar)))
            .with_key(self.id.clone())
    }
}
