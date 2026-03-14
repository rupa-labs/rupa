use rupa_core::{VNode, Vec2, Color, Renderer};
use rupa_core::scene::layout::LayoutEngine;
use rupa_core::scene::SceneNode;
use rupa_tui::TerminalRenderer;

pub struct Painter;

impl Painter {
    pub fn paint_vnode(
        renderer: &mut TerminalRenderer,
        node: &VNode,
        layout_engine: &LayoutEngine,
        scene_node: SceneNode,
        global_pos: Vec2,
        focused_id: Option<&str>,
        inherited_style: rupa_core::renderer::TypographyStyle,
    ) {
        // Use the decoupled LayoutEngine to get physical coordinates
        let pos = global_pos + layout_engine.get_physical_position(scene_node);
        let size = layout_engine.get_physical_size(scene_node);
        
        let rx = pos.x;
        let ry = pos.y;
        let rw = size.x;
        let rh = size.y;

        match node {
            VNode::Element(el) => {
                let is_focused = el.key.as_deref() == focused_id && focused_id.is_some();
                let is_input = el.tag == "input";
                let is_button = el.tag == "button" || el.handlers.on_click.is_some();
                
                let mut current_typography = el.style.typography.clone();
                if current_typography.color.is_none() {
                    current_typography.color = inherited_style.color;
                }

                let mut color = el.style.background.color.as_ref().map(|c| c.to_rgba());
                
                if is_focused {
                    if is_input {
                        color = Some([0.05, 0.05, 0.1, 1.0]);
                    } else if is_button {
                        color = Some([0.1, 0.3, 0.5, 1.0]); 
                        current_typography.color = Some(Color::Rgba(1.0, 1.0, 1.0, 1.0));
                    }
                }

                if let Some(rgba) = color {
                    renderer.draw_rect(rx, ry, rw, rh, rgba, 0.0);
                }

                if el.style.border.width != rupa_core::vnode::Unit::Absolute(0.0) || is_focused {
                    let border_color = if is_focused { 
                        if is_input { [1.0, 1.0, 0.0, 1.0] } else { [0.0, 0.8, 1.0, 1.0] } 
                    } else { 
                        [0.2, 0.2, 0.2, 1.0] 
                    };
                    renderer.draw_outline(rx, ry, rw, rh, border_color);

                    if let Some(ref label_text) = el.label {
                        let mut label_style = rupa_core::renderer::TypographyStyle::default();
                        label_style.color = Some(Color::Rgba(border_color[0], border_color[1], border_color[2], border_color[3]));
                        
                        let label_len = label_text.len() as f32;
                        let lx = match el.label_align {
                            rupa_core::vnode::TextAlign::Left => rx + 2.0,
                            rupa_core::vnode::TextAlign::Center => rx + (rw - label_len) / 2.0,
                            rupa_core::vnode::TextAlign::Right => rx + rw - label_len - 2.0,
                            _ => rx + 2.0,
                        };
                        
                        let bg_color = color.unwrap_or([0.0, 0.0, 0.0, 0.0]);
                        renderer.draw_rect(lx - 1.0, ry, label_len + 2.0, 1.0, bg_color, 0.0);
                        renderer.draw_text(&format!(" {} ", label_text), lx - 1.0, ry, label_len + 2.0, &label_style);
                    }
                }

                // In a full implementation, we would get child SceneNodes from the solver/engine
                // For now, we'll keep the logic simple to demonstrate the architecture
            }
            VNode::Text(text) => {
                renderer.draw_text(text, rx, ry, rw, &inherited_style);
            }
            _ => {}
        }
    }
}
