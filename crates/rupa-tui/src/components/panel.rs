use rupa_core::{Component, VNode, ViewCore, Id};
use std::sync::Arc;

/// A terminal-specific panel with customizable borders.
pub struct Panel<'a> {
    pub id: String,
    pub title: String,
    pub children: Vec<Box<dyn Component + 'a>>,
    pub view: Arc<ViewCore>,
}

impl<'a> Panel<'a> {
    pub fn new(title: impl Into<String>) -> Self {
        let view = Arc::new(ViewCore::new());
        // Default terminal border style
        view.style().border.width = 1.0;
        Self {
            id: Id::next().to_string(),
            title: title.into(),
            children: Vec::new(),
            view,
        }
    }

    pub fn child(mut self, child: impl Component + 'a) -> Self {
        self.children.push(Box::new(child));
        self.view.mark_dirty();
        self
    }
}

impl<'a> Component for Panel<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> {
        self.children.iter().map(|c| c.as_ref()).collect()
    }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }

    fn render(&self) -> VNode {
        VNode::element("panel")
            .with_style(self.view.style().clone())
            .with_attr("title", self.title.clone())
            .with_children(self.children.iter().map(|c| c.render()).collect())
            .with_key(self.id.clone())
            .with_label(self.title.clone()) // Automatically use title as border label
    }
}
