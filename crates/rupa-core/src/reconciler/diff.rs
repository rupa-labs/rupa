use rupa_vnode::VNode;
use super::patch::{Patch, UpdateType, PatchSet};
use crate::component::Component;
use rupa_signals::with_context;
use std::sync::Arc;

/// Orchestrates the reconciliation of a component, using its dirty flag 
/// to skip unnecessary work.
pub fn reconcile_component(comp: &dyn Component) -> PatchSet {
    let view = comp.view_core();
    
    // 1. If the component is clean, skip its own render and just reconcile its children
    if !view.is_dirty() {
        let mut patches = Vec::new();
        for (i, child) in comp.children().iter().enumerate() {
            patches.extend(reconcile_component(*child));
        }
        return patches;
    }

    // 2. Component is dirty: perform a full render with reactive tracking
    let old_vnode = view.get_prev_vnode();
    
    // Track dependencies during render: if any signal accessed here changes, 
    // it will call view.notify() which marks it as dirty.
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
pub fn reconcile(old: &VNode, new: &VNode, parent_id: Option<String>, index: usize) -> PatchSet {
    let mut patches = Vec::new();

    match (old, new) {
        // 1. Tag Mismatch -> Replace Entire Sub-tree
        (VNode::Element(old_el), VNode::Element(new_el)) if old_el.tag != new_el.tag => {
            patches.push(Patch::Replace {
                id: old_el.key.clone().unwrap_or_else(|| format!("{}_{}", old_el.tag, index)),
                new_node: new.clone(),
            });
        }

        // 2. Element Update -> Diff Styles and Attributes
        (VNode::Element(old_el), VNode::Element(new_el)) => {
            let mut changes = Vec::new();
            let el_id = old_el.key.clone().unwrap_or_else(|| format!("{}_{}", old_el.tag, index));

            if old_el.style != new_el.style {
                changes.push(UpdateType::Style(new_el.style.clone()));
            }

            // TODO: Deep Attribute Diffing
            if old_el.attributes != new_el.attributes {
                // Simplified for this artisan MVP
                for (key, value) in &new_el.attributes.map {
                    changes.push(UpdateType::Attribute(key.clone(), value.clone()));
                }
            }

            if !changes.is_empty() {
                patches.push(Patch::Update { id: el_id, changes });
            }

            // 3. Child Reconciliation (Simplified Index-based)
            let max_len = old_el.children.len().max(new_el.children.len());
            for i in 0..max_len {
                match (old_el.children.get(i), new_el.children.get(i)) {
                    (Some(o), Some(n)) => {
                        patches.extend(reconcile(o, n, Some(new_el.tag.clone()), i));
                    }
                    (None, Some(n)) => {
                        patches.push(Patch::Create {
                            parent_id: Some(new_el.tag.clone()), // Should use real ID
                            node: n.clone(),
                            index: i,
                        });
                    }
                    (Some(_o), None) => {
                        patches.push(Patch::Delete {
                            id: format!("{}_child_{}", new_el.tag, i),
                        });
                    }
                }
            }
        }

        // 4. Text Update
        (VNode::Text(old_txt), VNode::Text(new_txt)) if old_txt != new_txt => {
            patches.push(Patch::Update {
                id: format!("text_{}", index),
                changes: vec![UpdateType::Text(new_txt.clone())],
            });
        }

        // 5. Type Mismatch (e.g. Element to Text) -> Replace
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
