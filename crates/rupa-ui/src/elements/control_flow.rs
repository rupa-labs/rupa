use rupa_core::{Component, VNode, ViewCore, Id, Signal, Renderer, TextMeasurer, SceneNode, Vec2};
use taffy::prelude::*;
use std::sync::Arc;

// --- SHOW ---

pub struct ShowLogic {
    pub condition: Signal<bool>,
    pub fallback: VNode,
}

pub struct ShowView {
    pub core: Arc<ViewCore>,
}

pub struct Show<'a> {
    pub id: String,
    pub logic: ShowLogic,
    pub view: ShowView,
    pub child: Box<dyn Component + 'a>,
}

impl<'a> Show<'a> {
    pub fn new(condition: Signal<bool>, child: impl Component + 'a) -> Self {
        Self {
            id: Id::next().to_string(),
            logic: ShowLogic { condition, fallback: VNode::Empty },
            view: ShowView { core: Arc::new(ViewCore::new()) },
            child: Box::new(child),
        }
    }
}

impl<'a> Component for Show<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![self.child.as_ref()] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        if self.logic.condition.get() {
            self.child.render()
        } else {
            self.logic.fallback.clone()
        }
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        if self.logic.condition.get() {
            self.child.layout(taffy, measurer, parent)
        } else {
            let node = taffy.new_leaf(taffy::Style::default()).unwrap();
            self.view.core.set_node(SceneNode::from(node));
            node
        }
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        if self.logic.condition.get() {
            self.child.paint(renderer, taffy, node, is_group_hovered, global_pos);
        }
    }
}

// --- FOR EACH ---

pub struct ForEachLogic<T> {
    pub items: Signal<Vec<T>>,
}

pub struct ForEachView {
    pub core: Arc<ViewCore>,
}

pub struct ForEach<'a, T> {
    pub id: String,
    pub logic: ForEachLogic<T>,
    pub view: ForEachView,
    pub template: Arc<dyn Fn(&T) -> Box<dyn Component + 'a> + Send + Sync>,
}

impl<'a, T: 'static + Send + Sync + Clone> ForEach<'a, T> {
    pub fn new(items: Signal<Vec<T>>, template: impl Fn(&T) -> Box<dyn Component + 'a> + Send + Sync + 'static) -> Self {
        Self {
            id: Id::next().to_string(),
            logic: ForEachLogic { items },
            view: ForEachView { core: Arc::new(ViewCore::new()) },
            template: Arc::new(template),
        }
    }
}

impl<'a, T: 'static + Send + Sync + Clone> Component for ForEach<'a, T> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        let mut children = Vec::new();
        for item in self.logic.items.get().iter() {
            let comp = (self.template)(item);
            children.push(comp.render());
        }
        VNode::Fragment(children)
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, _taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        // Complex logic for fragment reconciliation
        todo!("ForEach layout reconciliation pending")
    }

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}
