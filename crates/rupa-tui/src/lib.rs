//! # Rupa Terminal UI (TUI) 📟
//!
//! Terminal UI orchestration and rendering for the Rupa Framework. 
//! This crate provides the **Composites** for transforming `VNode` trees 
//! into aesthetic, ANSI-powered terminal experiences.

use rupa_core::{Renderer, RenderCore, TextMeasurer, vnode::TextAlign};
use rupa_base::Vec2;
use std::io::{Write, stdout, Stdout};
use std::sync::{Arc, RwLock};

pub mod components;
pub use components::*;

/// A high-performance TUI renderer for the Rupa Framework.
/// 
/// Uses true-color ANSI escape sequences to render the universal UI tree 
/// onto a terminal grid. Supports double-buffering and surgical updates.
pub struct TerminalRenderer {
    /// The underlying render state and layout results.
    pub core: RenderCore,
    stdout: Stdout,
    buffer: String,
}

impl TerminalRenderer {
    /// Creates a new renderer with the specified grid dimensions.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            core: RenderCore::new(width, height, 1.0),
            stdout: stdout(),
            buffer: String::with_capacity(4096),
        }
    }

    /// Flushes the internal buffer to the terminal stdout.
    pub fn flush(&mut self) {
        let _ = self.stdout.write_all(self.buffer.as_bytes());
        let _ = self.stdout.flush();
        self.buffer.clear();
    }

    /// Clears the terminal screen and resets cursor position.
    pub fn clear_screen(&mut self) {
        self.buffer.push_str("\x1B[2J\x1B[H");
    }

    /// Moves the terminal cursor to a specific character cell.
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
        Vec2::new(text.len() as f32, 1.0)
    }
}

impl Renderer for TerminalRenderer {
    fn core(&self) -> &RenderCore { &self.core }
    fn core_mut(&mut self) -> &mut RenderCore { &mut self.core }

    fn render_patch(&mut self, _patch: rupa_core::PatchSet) {
        // High-level patch application logic
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

        self.move_cursor(x, y);
        self.buffer.push_str(&ansi_color);
        self.buffer.push_str("┌");
        self.move_cursor(x + w - 1, y);
        self.buffer.push_str("┐");
        self.move_cursor(x, y + h - 1);
        self.buffer.push_str("└");
        self.move_cursor(x + w - 1, y + h - 1);
        self.buffer.push_str("┘");

        for i in 1..(w - 1) {
            self.move_cursor(x + i, y);
            self.buffer.push_str("─");
            self.move_cursor(x + i, y + h - 1);
            self.buffer.push_str("─");
        }

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

/// A mock helper for testing TUI rendering in headless environments.
pub struct MockTerminal {
    pub output: Arc<RwLock<Vec<String>>>,
}

impl MockTerminal {
    /// Creates a new, empty mock terminal tracker.
    pub fn new() -> Self {
        Self {
            output: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_conversion() {
        let ansi = TerminalRenderer::color_to_ansi([1.0, 0.0, 0.0, 1.0], false);
        assert!(ansi.contains("38;2;255;0;0"));
    }
}
