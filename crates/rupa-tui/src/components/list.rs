use rupa_core::{Component, VNode, ViewCore, Id, Signal, UIEvent, renderer::{Renderer, TextMeasurer}, scene::SceneNode};
use rupa_ui::elements::{VStack, Text};
use taffy::prelude::*;
use std::sync::Arc;

/// A TUI-optimized selection list.
pub struct List {
    pub id: String,
    pub options: Vec<String>,
    pub selected_index: Signal<usize>,
    pub view: Arc<ViewCore>,
    pub on_submit: Option<Arc<dyn Fn(usize) + Send + Sync>>,
}

impl List {
    pub fn new(options: Vec<impl Into<String>>) -> Self {
        Self {
            id: Id::next().to_string(),
            options: options.into_iter().map(|s| s.into()).collect(),
            selected_index: Signal::new(0),
            view: Arc::new(ViewCore::new()),
            on_submit: None,
        }
    }

    pub fn on_submit(mut self, f: impl Fn(usize) + Send + Sync + 'static) -> Self {
        self.on_submit = Some(Arc::new(f));
        self
    }
}

impl Component for List {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }

    fn render(&self) -> VNode {
        let selected = self.selected_index.get();
        let mut list = VStack::new();

        for (i, opt) in self.options.iter().enumerate() {
            let is_selected = i == selected;
            let prefix = if is_selected { " > " } else { "   " };
            let text = format!("{}{}", prefix, opt);
            
            list = list.child(Box::new(Text::new(text)));
        }

        list.render()
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.style().to_taffy()).unwrap();
        self.view.set_node(SceneNode::from(node));
        node
    }

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: rupa_base::Vec2) {}

    fn on_key(&self, _event: &mut UIEvent, key: rupa_core::events::KeyCode) {
        match key {
            rupa_core::events::KeyCode::ArrowUp => {
                let cur = self.selected_index.get();
                if cur > 0 { self.selected_index.set(cur - 1); }
            }
            rupa_core::events::KeyCode::ArrowDown => {
                let cur = self.selected_index.get();
                if cur < self.options.len() - 1 { self.selected_index.set(cur + 1); }
            }
            rupa_core::events::KeyCode::Enter => {
                if let Some(ref cb) = self.on_submit {
                    (cb)(self.selected_index.get());
                }
            }
            _ => {}
        }
    }
}
