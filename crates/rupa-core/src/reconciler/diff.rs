use rupa_vnode::VNode;
use rupa_vnode::style::Style;
use super::patch::{Patch, UpdateType, PatchSet, StylePart};
use crate::component::Component;
use rupa_signals::with_context;
use std::sync::Arc;
use std::collections::HashMap;

/// Orchestrates the reconciliation of a component, using its dirty flag 
/// to skip unnecessary work.
pub fn reconcile_component(comp: &dyn Component) -> PatchSet {
    let view = comp.view_core();
    
    if !view.is_dirty() {
        let mut patches = Vec::new();
        for (_i, child) in comp.children().iter().enumerate() {
            patches.extend(reconcile_component(*child));
        }
        return patches;
    }

    let old_vnode = view.get_prev_vnode();
    let new_vnode = with_context(view.clone() as Arc<dyn rupa_signals::Subscriber>, || {
        comp.render()
    });

    let patches = reconcile(&old_vnode, &new_vnode, None, 0);
    
    view.set_prev_vnode(new_vnode);
    view.clear_dirty();
    
    patches
}

/// Compares two VNode trees and identifies the minimal set of instructions 
/// to transform the old tree into the new one.
pub fn reconcile(old: &VNode, new: &VNode, _parent_id: Option<String>, index: usize) -> PatchSet {
    let mut patches = Vec::new();

    match (old, new) {
        (VNode::Element(old_el), VNode::Element(new_el)) if old_el.tag != new_el.tag => {
            patches.push(Patch::Replace {
                id: old_el.key.clone().unwrap_or_else(|| format!("{}_{}", old_el.tag, index)),
                new_node: new.clone(),
            });
        }

        (VNode::Element(old_el), VNode::Element(new_el)) => {
            let mut changes = Vec::new();
            let el_id = old_el.key.clone().unwrap_or_else(|| format!("{}_{}", old_el.tag, index));

            // 1. Deep Style Diffing (with Motion awareness)
            diff_styles(&old_el.style, &new_el.style, &mut changes);

            // 2. Deep Attribute Diffing
            diff_attributes(&old_el.attributes, &new_el.attributes, &mut changes);

            if !changes.is_empty() {
                // If the element has motion rules, we can wrap the updates later
                // (Future: specialized animated patches)
                patches.push(Patch::Update { id: el_id.clone(), changes });
            }

            // 3. Keyed Child Reconciliation
            reconcile_children(&old_el.children, &new_el.children, Some(el_id), &mut patches);
        }

        (VNode::Text(old_txt), VNode::Text(new_txt)) if old_txt != new_txt => {
            patches.push(Patch::Update {
                id: format!("text_{}", index),
                changes: vec![UpdateType::Text(new_txt.clone())],
            });
        }

        _ if old != new => {
            patches.push(Patch::Replace {
                id: format!("node_{}", index),
                new_node: new.clone(),
            });
        }

        _ => {}
    }

    patches
}

fn diff_styles(old: &Style, new: &Style, changes: &mut Vec<UpdateType>) {
    if old.layout != new.layout { changes.push(UpdateType::StylePart(StylePart::Layout(new.layout.clone()))); }
    if old.flex != new.flex { changes.push(UpdateType::StylePart(StylePart::Flex(new.flex.clone()))); }
    if old.grid != new.grid { changes.push(UpdateType::StylePart(StylePart::Grid(new.grid.clone()))); }
    if old.sizing != new.sizing { changes.push(UpdateType::StylePart(StylePart::Sizing(new.sizing.clone()))); }
    if old.padding != new.padding { changes.push(UpdateType::StylePart(StylePart::Padding(new.padding.clone()))); }
    if old.margin != new.margin { changes.push(UpdateType::StylePart(StylePart::Margin(new.margin.clone()))); }
    if old.background != new.background { changes.push(UpdateType::StylePart(StylePart::Background(new.background.clone()))); }
    if old.border != new.border { changes.push(UpdateType::StylePart(StylePart::Border(new.border.clone()))); }
    if old.rounding != new.rounding { changes.push(UpdateType::StylePart(StylePart::Rounding(new.rounding.clone()))); }
    if old.outline != new.outline { changes.push(UpdateType::StylePart(StylePart::Outline(new.outline.clone()))); }
    if old.typography != new.typography { changes.push(UpdateType::StylePart(StylePart::Typography(new.typography.clone()))); }
    if old.interactivity != new.interactivity { changes.push(UpdateType::StylePart(StylePart::Interactivity(new.interactivity.clone()))); }
    if old.shadow != new.shadow { changes.push(UpdateType::StylePart(StylePart::Shadow(new.shadow.clone()))); }
    if old.filter != new.filter { changes.push(UpdateType::StylePart(StylePart::Filter(new.filter.clone()))); }
    if old.motion != new.motion { changes.push(UpdateType::StylePart(StylePart::Motion(new.motion.clone()))); }
}

fn diff_attributes(old: &rupa_vnode::Attributes, new: &rupa_vnode::Attributes, changes: &mut Vec<UpdateType>) {
    for (key, value) in &new.map {
        if old.map.get(key) != Some(value) {
            changes.push(UpdateType::Attribute(key.clone(), value.clone()));
        }
    }
    for key in old.map.keys() {
        if !new.map.contains_key(key) {
            changes.push(UpdateType::RemoveAttribute(key.clone()));
        }
    }
}

fn reconcile_children(old_children: &[VNode], new_children: &[VNode], _parent_id: Option<String>, patches: &mut Vec<Patch>) {
    let mut old_keyed = HashMap::new();
    let mut old_unkeyed = Vec::new();

    for (i, child) in old_children.iter().enumerate() {
        if let VNode::Element(el) = child {
            if let Some(key) = &el.key {
                old_keyed.insert(key.clone(), (i, child));
                continue;
            }
        }
        old_unkeyed.push((i, child));
    }

    let mut old_unkeyed_iter = old_unkeyed.into_iter();

    for (i, new_child) in new_children.iter().enumerate() {
        if let VNode::Element(new_el) = new_child {
            if let Some(key) = &new_el.key {
                if let Some((old_idx, old_child)) = old_keyed.remove(key) {
                    if old_idx != i {
                        patches.push(Patch::Move {
                            id: key.clone(),
                            from_index: old_idx,
                            to_index: i,
                        });
                    }
                    patches.extend(reconcile(old_child, new_child, _parent_id.clone(), i));
                } else {
                    patches.push(Patch::Create {
                        parent_id: _parent_id.clone(),
                        node: new_child.clone(),
                        index: i,
                    });
                }
                continue;
            }
        }

        // Fallback to unkeyed matching by index
        if let Some((old_idx, old_child)) = old_unkeyed_iter.next() {
            patches.extend(reconcile(old_child, new_child, _parent_id.clone(), old_idx));
        } else {
            patches.push(Patch::Create {
                parent_id: _parent_id.clone(),
                node: new_child.clone(),
                index: i,
            });
        }
    }

    // Clean up remaining old nodes
    for (_, (old_idx, old_child)) in old_keyed {
        let id = if let VNode::Element(el) = old_child {
            el.key.clone().unwrap_or_else(|| format!("node_{}", old_idx))
        } else {
            format!("node_{}", old_idx)
        };
        patches.push(Patch::Delete { id });
    }
    for (old_idx, old_child) in old_unkeyed_iter {
        let id = if let VNode::Element(el) = old_child {
            el.key.clone().unwrap_or_else(|| format!("node_{}", old_idx))
        } else {
            format!("node_{}", old_idx)
        };
        patches.push(Patch::Delete { id });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rupa_vnode::VNode;
    use rupa_vnode::style::Style;
    use rupa_vnode::Attributes;

    #[test]
    fn test_attribute_diffing() {
        let old_node = VNode::element("div")
            .with_key("test_node")
            .with_attr("id", "test")
            .with_attr("class", "old");

        let new_node = VNode::element("div")
            .with_key("test_node")
            .with_attr("id", "test")
            .with_attr("class", "new")
            .with_attr("data-foo", "bar");

        let patches = reconcile(&old_node, &new_node, None, 0);
        
        assert!(!patches.is_empty());
        
        if let Patch::Update { id: _, changes } = &patches[0] {
            let has_id_update = changes.iter().any(|c| match c {
                UpdateType::Attribute(k, _) => k == "id",
                _ => false,
            });
            assert!(!has_id_update, "Should NOT update unchanged attribute 'id'");
            
            let has_class_update = changes.iter().any(|c| match c {
                UpdateType::Attribute(k, v) => k == "class" && v == "new",
                _ => false,
            });
            assert!(has_class_update, "Should update changed attribute 'class'");

            let has_foo_update = changes.iter().any(|c| match c {
                UpdateType::Attribute(k, v) => k == "data-foo" && v == "bar",
                _ => false,
            });
            assert!(has_foo_update, "Should add new attribute 'data-foo'");
        } else {
            panic!("Expected Patch::Update");
        }
    }

    #[test]
    fn test_style_part_diffing() {
        let old_style = Style::default().p(10.0);
        let new_style = Style::default().p(20.0).w(100.0);
        
        let old_node = VNode::element("div").with_style(old_style);
        let new_node = VNode::element("div").with_style(new_style);

        let patches = reconcile(&old_node, &new_node, None, 0);
        
        if let Patch::Update { id: _, changes } = &patches[0] {
            let has_padding_update = changes.iter().any(|c| match c {
                UpdateType::StylePart(StylePart::Padding(_)) => true,
                _ => false,
            });
            assert!(has_padding_update, "Should identify Padding change");

            let has_sizing_update = changes.iter().any(|c| match c {
                UpdateType::StylePart(StylePart::Sizing(_)) => true,
                _ => false,
            });
            assert!(has_sizing_update, "Should identify Sizing change");
        }
    }

    #[test]
    fn test_keyed_reconciliation_move() {
        let child1 = VNode::element("span").with_key("1");
        let child2 = VNode::element("span").with_key("2");
        
        let old_node = VNode::element("div").with_child(child1.clone()).with_child(child2.clone());
        let new_node = VNode::element("div").with_child(child2.clone()).with_child(child1.clone());

        let patches = reconcile(&old_node, &new_node, None, 0);
        
        let has_move = patches.iter().any(|p| matches!(p, Patch::Move { .. }));
        assert!(has_move, "Should use Patch::Move for keyed elements");
    }
}
