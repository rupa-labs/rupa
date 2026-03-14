use rupa_core::{Component, VNode, Id, Signal, events::KeyCode};
use rupa_vnode::Style;
use rupa_ui::elements::*;
use rupa_ui::style::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// A TUI-optimized selection list.
pub struct List {
    pub id: String,
    pub options: Vec<String>,
    pub selected_index: Signal<usize>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
    pub on_submit: Option<Arc<dyn Fn(usize) + Send + Sync>>,
}

impl List {
    pub fn new(options: Vec<impl Into<String>>) -> Self {
        Self {
            id: Id::next().to_string(),
            options: options.into_iter().map(|s| s.into()).collect(),
            selected_index: Signal::new(0),
            style: Arc::new(RwLock::new(Style::default())),
            prev_vnode: Arc::new(RwLock::new(None)),
            on_submit: None,
        }
    }

    pub fn on_submit(mut self, f: impl Fn(usize) + Send + Sync + 'static) -> Self {
        self.on_submit = Some(Arc::new(f));
        self
    }
}

impl Component for List {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }

    fn render(&self) -> VNode {
        let selected = self.selected_index.get();
        let selected_index = self.selected_index.clone();
        let options_len = self.options.len();
        let on_submit_cb = self.on_submit.clone();

        let mut list_node = VNode::element("div")
            .with_style(self.style.read().unwrap().clone())
            .with_key(self.id.clone());

        let mut children = Vec::new();
        for (i, opt) in self.options.iter().enumerate() {
            let is_selected = i == selected;
            let prefix = if is_selected { " > " } else { "   " };
            let text = format!("{}{}", prefix, opt);
            
            children.push(VNode::element("span")
                .with_child(VNode::text(text))
                .with_key(format!("{}_opt_{}", self.id, i)));
        }

        list_node = list_node.with_children(children);

        // Add keyboard navigation
        list_node = list_node.on_key_down(move |_ui_ev, key| {
            match key {
                KeyCode::ArrowUp => {
                    let cur = selected_index.get();
                    if cur > 0 { selected_index.set(cur - 1); }
                }
                KeyCode::ArrowDown => {
                    let cur = selected_index.get();
                    if cur < options_len - 1 { selected_index.set(cur + 1); }
                }
                KeyCode::Enter => {
                    if let Some(ref cb) = on_submit_cb {
                        (cb)(selected_index.get());
                    }
                }
                _ => {}
            }
        });

        list_node
    }
}

impl Stylable for List {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
