use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::Attributes;
use taffy::prelude::*;
use std::sync::Arc;

// --- SVG ---

pub struct SvgLogic {
    pub content: String,
}

pub struct SvgView {
    pub core: Arc<ViewCore>,
}

pub struct Svg {
    pub id: String,
    pub logic: SvgLogic,
    pub view: SvgView,
}

impl Svg {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            logic: SvgLogic { content: content.into() },
            view: SvgView { core: Arc::new(ViewCore::new()) },
        }
    }
}

impl Component for Svg {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "svg".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("content", self.logic.content.clone());
                attr
            },
            children: vec![],
            key: Some(self.id.clone()),
        })
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.core.style().to_taffy()).unwrap();
        self.view.core.set_node(SceneNode::from(node));
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        node
    }

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}

// --- ICON ---

pub struct Icon { pub id: String, pub name: String, pub view: Arc<ViewCore> }
impl Icon {
    pub fn new(name: impl Into<String>) -> Self {
        Self { id: generate_id(), name: name.into(), view: Arc::new(ViewCore::new()) }
    }
}
impl Component for Icon {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode { VNode::element("icon") }
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
    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}
