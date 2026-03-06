use crate::Component;
use crate::renderer::Renderer;
use crate::utils::Vec2;
use taffy::prelude::*;

pub struct Children<'a> { pub list: Vec<Box<dyn Component + 'a>> }

impl<'a> Children<'a> {
    pub fn new() -> Self { Self { list: Vec::new() } }
    pub fn add(&mut self, child: Box<dyn Component + 'a>) { self.list.push(child); }
    pub fn append(&mut self, mut children: Vec<Box<dyn Component + 'a>>) { self.list.append(&mut children); }
    pub fn layout_all(&self, taffy: &mut TaffyTree<()>, parent: NodeId) -> Vec<NodeId> { self.list.iter().map(|child| child.layout(taffy, Some(parent))).collect() }
    pub fn paint_all(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, parent_node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>, parent_global_pos: Vec2, start_index: usize) {
        let children_nodes = taffy.children(parent_node).unwrap();
        for (i, child) in self.list.iter().enumerate() {
            if let Some(node) = children_nodes.get(i + start_index) {
                let layout = taffy.layout(*node).unwrap();
                let global_pos = parent_global_pos + Vec2::new(layout.location.x, layout.location.y);
                child.paint(renderer, taffy, *node, is_group_hovered, render_pass, global_pos);
            }
        }
    }


    pub fn dispatch_click(&self, taffy: &TaffyTree<()>, parent_node: NodeId, cursor_pos: Vec2) -> bool {
        let children_nodes = taffy.children(parent_node).unwrap();
        for (i, child) in self.list.iter().enumerate().rev() {
            if let Some(node) = children_nodes.get(i) {
                let layout = taffy.layout(*node).unwrap();
                if cursor_pos.x >= layout.location.x && cursor_pos.x <= layout.location.x + layout.size.width &&
                   cursor_pos.y >= layout.location.y && cursor_pos.y <= layout.location.y + layout.size.height {
                    child.on_click(); return true;
                }
            }
        }
        false
    }
    pub fn render_all(&self) { for child in &self.list { child.render(); } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::Text;
    #[test]
    fn test_children_append() { let mut children = Children::new(); children.add(Box::new(Text::new("One"))); assert_eq!(children.list.len(), 1); }
}
