use rupa_core::{VNode, Vec2, Color, Renderer};
use rupa_tui::TerminalRenderer;
use taffy::prelude::*;

pub struct Painter;

impl Painter {
    pub fn paint_vnode(
        renderer: &mut TerminalRenderer,
        node: &VNode,
        taffy: &TaffyTree<()>,
        layout_node: NodeId,
        global_pos: Vec2,
        focused_id: Option<&str>,
        inherited_style: rupa_core::renderer::TypographyStyle,
    ) {
        let layout = taffy.layout(layout_node).unwrap();
        let pos = global_pos + Vec2::new(layout.location.x, layout.location.y);
        
        let rx = pos.x.round();
        let ry = pos.y.round();
        let rw = layout.size.width.round();
        let rh = layout.size.height.round();

        match node {
            VNode::Element(el) => {
                let is_focused = el.key.as_deref() == focused_id && focused_id.is_some();
                let is_input = el.tag == "input";
                let is_button = el.tag == "button" || el.handlers.on_click.is_some();
                
                // Merge typography styles for children
                let mut current_typography = el.style.typography.clone();
                // Inherit color if not set on current element
                if current_typography.color.is_none() {
                    current_typography.color = inherited_style.color;
                }

                // Draw Background
                let mut color = el.style.background.color.as_ref().map(|c| c.to_rgba());
                
                // Artisan Focus Styling (Subtle & Semantic)
                if is_focused {
                    if is_input {
                        color = Some([0.05, 0.05, 0.1, 1.0]); // Very deep dark
                    } else if is_button {
                        // Subtle highlight instead of solid blue
                        color = Some([0.1, 0.3, 0.5, 1.0]); 
                        current_typography.color = Some(Color::Rgba(1.0, 1.0, 1.0, 1.0));
                    }
                }

                if let Some(rgba) = color {
                    renderer.draw_rect(rx, ry, rw, rh, rgba, 0.0);
                }

                // Border logic
                if el.style.border.width != 0.0 || is_focused {
                    let border_color = if is_focused { 
                        if is_input { [1.0, 1.0, 0.0, 1.0] } else { [0.0, 0.8, 1.0, 1.0] } 
                    } else { 
                        [0.2, 0.2, 0.2, 1.0] 
                    };
                    renderer.draw_outline(rx, ry, rw, rh, border_color);

                    // Draw Label on top of border
                    if let Some(ref label_text) = el.label {
                        let mut label_style = rupa_core::renderer::TypographyStyle::default();
                        label_style.color = Some(Color::Rgba(border_color[0], border_color[1], border_color[2], border_color[3]));
                        label_style.weight = if is_focused { rupa_core::vnode::FontWeight::Bold } else { rupa_core::vnode::FontWeight::Normal };
                        
                        let label_len = label_text.len() as f32;
                        let lx = match el.label_align {
                            rupa_core::vnode::TextAlign::Left => rx + 2.0,
                            rupa_core::vnode::TextAlign::Center => rx + (rw - label_len) / 2.0,
                            rupa_core::vnode::TextAlign::Right => rx + rw - label_len - 2.0,
                            _ => rx + 2.0,
                        };
                        
                        // Draw a small "hole" in the border for the label (by drawing background color rect)
                        let bg_color = color.unwrap_or([0.0, 0.0, 0.0, 0.0]);
                        renderer.draw_rect(lx - 1.0, ry, label_len + 2.0, 1.0, bg_color, 0.0);
                        
                        renderer.draw_text(&format!(" {} ", label_text), lx - 1.0, ry, label_len + 2.0, &label_style);
                    }
                }

                // Focus Indicators
                if is_focused {
                    if is_input {
                        let mut focus_style = rupa_core::renderer::TypographyStyle::default();
                        focus_style.color = Some(Color::Rgba(1.0, 1.0, 0.0, 1.0));
                        renderer.draw_text("┃", rx - 1.0, ry, 1.0, &focus_style);
                    } else if is_button {
                        let mut focus_style = rupa_core::renderer::TypographyStyle::default();
                        focus_style.color = Some(Color::Rgba(0.0, 1.0, 1.0, 1.0));
                        renderer.draw_text(">", rx - 2.0, ry, 1.0, &focus_style);
                    }
                }

                // Render Input Value with Cursor
                if is_input {
                    let val = el.attributes.get("value").cloned().unwrap_or_default();
                    let display_text = if is_focused { format!("{}_", val) } else { val };
                    let mut input_style = current_typography.clone();
                    input_style.color = Some(Color::Rgba(1.0, 1.0, 1.0, 1.0));
                    renderer.draw_text(&display_text, rx + 1.0, ry, rw - 2.0, &input_style);
                }

                let taffy_children = taffy.children(layout_node).unwrap();
                for (i, child) in el.children.iter().enumerate() {
                    if let Some(child_layout_node) = taffy_children.get(i) {
                        Self::paint_vnode(renderer, child, taffy, *child_layout_node, pos, focused_id, current_typography.clone());
                    }
                }
            }
            VNode::Text(text) => {
                renderer.draw_text(text, rx, ry, rw, &inherited_style);
            }
            VNode::Fragment(children) => {
                let taffy_children = taffy.children(layout_node).unwrap();
                for (i, child) in children.iter().enumerate() {
                    if let Some(child_layout_node) = taffy_children.get(i) {
                        Self::paint_vnode(renderer, child, taffy, *child_layout_node, pos, focused_id, inherited_style.clone());
                    }
                }
            }
            _ => {}
        }
    }
}
