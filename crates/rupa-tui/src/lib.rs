use rupa_core::{Renderer, RenderCore, TextMeasurer, vnode::TextAlign};
use rupa_support::Vec2;
use std::io::{Write, stdout, Stdout};

/// A high-performance TUI renderer for the Rupa Framework.
pub struct TerminalRenderer {
    pub core: RenderCore,
    stdout: Stdout,
    buffer: String,
}

impl TerminalRenderer {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            core: RenderCore::new(width, height, 1.0),
            stdout: stdout(),
            buffer: String::with_capacity(4096),
        }
    }

    pub fn flush(&mut self) {
        self.stdout.write_all(self.buffer.as_bytes()).unwrap();
        self.stdout.flush().unwrap();
        self.buffer.clear();
    }

    pub fn clear_screen(&mut self) {
        self.buffer.push_str("\x1B[2J\x1B[H");
    }

    pub fn move_cursor(&mut self, x: u16, y: u16) {
        self.buffer.push_str(&format!("\x1B[{};{}H", y + 1, x + 1));
    }

    fn color_to_ansi(color: [f32; 4], is_bg: bool) -> String {
        let r = (color[0] * 255.0) as u8;
        let g = (color[1] * 255.0) as u8;
        let b = (color[2] * 255.0) as u8;
        let code = if is_bg { 48 } else { 38 };
        format!("\x1B[{};2;{};{};{}m", code, r, g, b)
    }
}

impl TextMeasurer for TerminalRenderer {
    fn measure(&self, text: &str, _size: f32) -> Vec2 {
        // In TUI, each character is roughly 1x1 logical unit
        Vec2::new(text.len() as f32, 1.0)
    }
}

impl Renderer for TerminalRenderer {
    fn core(&self) -> &RenderCore { &self.core }
    fn core_mut(&mut self) -> &mut RenderCore { &mut self.core }

    fn render_patch(&mut self, _patch: rupa_core::Patch) {
        // TUI Patch execution logic
    }

    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], _radius: f32) {
        let ansi_color = Self::color_to_ansi(color, true);
        let x = x as u16;
        let y = y as u16;
        let w = w as u16;
        let h = h as u16;

        for row in 0..h {
            self.move_cursor(x, y + row);
            self.buffer.push_str(&ansi_color);
            for _ in 0..w {
                self.buffer.push_str(" ");
            }
            self.buffer.push_str("\x1B[0m");
        }
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32, _w: f32, _size: f32, color: [f32; 4], _align: TextAlign) {
        let ansi_color = Self::color_to_ansi(color, false);
        self.move_cursor(x as u16, y as u16);
        self.buffer.push_str(&ansi_color);
        self.buffer.push_str(text);
        self.buffer.push_str("\x1B[0m");
    }

    fn draw_outline(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        let ansi_color = Self::color_to_ansi(color, false);
        let x = x as u16;
        let y = y as u16;
        let w = w as u16;
        let h = h as u16;

        if w < 2 || h < 2 { return; }

        // Corners
        self.move_cursor(x, y);
        self.buffer.push_str(&ansi_color);
        self.buffer.push_str("┌");
        self.move_cursor(x + w - 1, y);
        self.buffer.push_str("┐");
        self.move_cursor(x, y + h - 1);
        self.buffer.push_str("└");
        self.move_cursor(x + w - 1, y + h - 1);
        self.buffer.push_str("┘");

        // Horizontal lines
        for i in 1..(w - 1) {
            self.move_cursor(x + i, y);
            self.buffer.push_str("─");
            self.move_cursor(x + i, y + h - 1);
            self.buffer.push_str("─");
        }

        // Vertical lines
        for j in 1..(h - 1) {
            self.move_cursor(x, y + j);
            self.buffer.push_str("│");
            self.move_cursor(x + w - 1, y + j);
            self.buffer.push_str("│");
        }
        
        self.buffer.push_str("\x1B[0m");
    }

    fn push_clip(&mut self, _x: f32, _y: f32, _w: f32, _h: f32) {}
    fn pop_clip(&mut self) {}

    fn present(&mut self) {
        self.flush();
    }
}

impl TerminalRenderer {
    /// Recursively paints a VNode tree onto the terminal buffer.
    pub fn paint_vnode(
        &mut self,
        node: &rupa_vnode::VNode,
        taffy: &taffy::prelude::TaffyTree<()>,
        layout_node: taffy::prelude::NodeId,
        global_pos: Vec2,
    ) {
        let layout = taffy.layout(layout_node).unwrap();
        let pos = global_pos + Vec2::new(layout.location.x, layout.location.y);

        match node {
            rupa_vnode::VNode::Element(el) => {
                // 1. Draw Background
                if let Some(ref color) = el.style.background.color {
                    self.draw_rect(pos.x, pos.y, layout.size.width, layout.size.height, color.to_rgba(), 0.0);
                }

                // 2. Draw Outline/Border (Artisan Style)
                if !el.style.border.width.is_zero() {
                    self.draw_outline(pos.x, pos.y, layout.size.width, layout.size.height, [0.5, 0.5, 0.5, 1.0]);
                }

                // 3. Draw Children
                let taffy_children = taffy.children(layout_node).unwrap();
                for (i, child) in el.children.iter().enumerate() {
                    if let Some(child_layout_node) = taffy_children.get(i) {
                        self.paint_vnode(child, taffy, *child_layout_node, pos);
                    }
                }
            }
            rupa_vnode::VNode::Text(text) => {
                let color = [1.0, 1.0, 1.0, 1.0]; // Default white for TUI text
                self.draw_text(text, pos.x, pos.y, layout.size.width, 1.0, color, TextAlign::Left);
            }
            rupa_vnode::VNode::Fragment(children) => {
                let taffy_children = taffy.children(layout_node).unwrap();
                for (i, child) in children.iter().enumerate() {
                    if let Some(child_layout_node) = taffy_children.get(i) {
                        self.paint_vnode(child, taffy, *child_layout_node, pos);
                    }
                }
            }
            _ => {}
        }
    }
}
pub mod runner;
pub use runner::TerminalRunner;
