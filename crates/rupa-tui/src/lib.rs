//! # Rupa Terminal UI (TUI) 📟
//!
//! Terminal UI orchestration and rendering for the Rupa Framework. 
//! This crate provides the **Composites** for transforming `VNode` trees 
//! into aesthetic, ANSI-powered terminal experiences.

use rupa_core::{Renderer, RenderCore, TextMeasurer};
use rupa_base::Vec2;
use rupa_vnode::style::typography::{TextDecoration, TextTransform};
use std::io::{Write, stdout, Stdout};
use std::sync::{Arc, RwLock};

pub mod components;
pub use components::*;

/// Represents a single character cell on the terminal grid.
#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub symbol: String,
    pub fg: [f32; 4],
    pub bg: [f32; 4],
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
    /// Bitmask for border connections: 1: Up, 2: Down, 4: Left, 8: Right
    pub border_mask: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: " ".to_string(),
            fg: [0.9, 0.9, 0.9, 1.0],
            bg: [0.0, 0.0, 0.0, 0.0],
            bold: false,
            italic: false,
            underline: false,
            strikethrough: false,
            border_mask: 0,
        }
    }
}

/// A high-performance TUI renderer for the Rupa Framework.
/// 
/// Uses a grid-based buffer system to ensure precise layout and 
/// double-buffering to eliminate flickering.
pub struct TerminalRenderer {
    pub core: RenderCore,
    stdout: Stdout,
    /// The current frame buffer being drawn to.
    cells: Vec<Cell>,
    /// The previous frame buffer used for diffing.
    last_cells: Vec<Cell>,
    /// Stack of clipping rectangles (x, y, w, h)
    clip_stack: Vec<[f32; 4]>,
}

impl TerminalRenderer {
    /// Creates a new renderer with the specified grid dimensions.
    pub fn new(width: f32, height: f32) -> Self {
        let w = width.round() as usize;
        let h = height.round() as usize;
        Self {
            core: RenderCore::new(width, height, 1.0),
            stdout: stdout(),
            cells: vec![Cell::default(); w * h],
            last_cells: vec![Cell::default(); w * h],
            clip_stack: Vec::new(),
        }
    }

    /// Resizes the internal grid buffers.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.core.logical_size = Vec2::new(width, height);
        let w = width.round() as usize;
        let h = height.round() as usize;
        self.cells = vec![Cell::default(); w * h];
        self.last_cells = vec![Cell::default(); w * h];
    }

    /// Clears the current frame buffer (resetting to default cells).
    pub fn clear_screen(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }

    fn color_to_ansi(color: [f32; 4], is_bg: bool) -> String {
        let r = (color[0] * 255.0) as u8;
        let g = (color[1] * 255.0) as u8;
        let b = (color[2] * 255.0) as u8;
        let code = if is_bg { 48 } else { 38 };
        format!("\x1B[{};2;{};{};{}m", code, r, g, b)
    }

    fn is_clipped(&self, x: i32, y: i32) -> bool {
        if let Some(clip) = self.clip_stack.last() {
            let cx = clip[0].round() as i32;
            let cy = clip[1].round() as i32;
            let cw = clip[2].round() as i32;
            let ch = clip[3].round() as i32;
            x < cx || x >= cx + cw || y < cy || y >= cy + ch
        } else {
            false
        }
    }
}

use unicode_width::UnicodeWidthStr;

impl TextMeasurer for TerminalRenderer {
    fn measure(&self, text: &str, _size: f32) -> Vec2 {
        Vec2::new(text.width() as f32, 1.0)
    }
}

impl Renderer for TerminalRenderer {
    fn core(&self) -> &RenderCore { &self.core }
    fn core_mut(&mut self) -> &mut RenderCore { &mut self.core }

    fn render_patch(&mut self, _patch: rupa_core::Patch) {}

    fn draw_rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4], _radius: f32) {
        let x_start = x.round() as i32;
        let y_start = y.round() as i32;
        let width = w.round() as i32;
        let height = h.round() as i32;
        
        let term_w = self.core.logical_size.x.round() as i32;
        let term_h = self.core.logical_size.y.round() as i32;

        for row in 0..height {
            for col in 0..width {
                let cx = x_start + col;
                let cy = y_start + row;
                
                if cx >= 0 && cx < term_w && cy >= 0 && cy < term_h {
                    if self.is_clipped(cx, cy) { continue; }
                    let idx = (cy * term_w + cx) as usize;
                    if idx < self.cells.len() {
                        // Blend background if alpha < 1.0 (Simple overwrite for now)
                        if color[3] > 0.0 {
                            self.cells[idx].bg = color;
                        }
                    }
                }
            }
        }
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32, w: f32, style: &rupa_core::renderer::TypographyStyle) {
        let start_x = x.round() as i32;
        let start_y = y.round() as i32;
        let max_w = w.round() as i32;
        
        let term_w = self.core.logical_size.x.round() as i32;
        let term_h = self.core.logical_size.y.round() as i32;

        let fg_color = style.color.as_ref().map(|c| c.to_rgba()).unwrap_or([0.9, 0.9, 0.9, 1.0]);

        // Apply Transform
        let transformed_text = match style.transform {
            TextTransform::Uppercase => text.to_uppercase(),
            TextTransform::Lowercase => text.to_lowercase(),
            TextTransform::Capitalize => {
                let mut c = text.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                }
            }
            _ => text.to_string(),
        };

        for (i, ch) in transformed_text.chars().enumerate() {
            let cx = start_x + i as i32;
            let cy = start_y;
            
            if cx >= 0 && cx < term_w && cy >= 0 && cy < term_h && (i as i32) < max_w {
                if self.is_clipped(cx, cy) { continue; }
                let idx = (cy * term_w + cx) as usize;
                if idx < self.cells.len() {
                    let cell = &mut self.cells[idx];
                    cell.symbol = ch.to_string();
                    cell.fg = fg_color;
                    cell.bold = style.weight >= rupa_vnode::FontWeight::Bold;
                    cell.italic = style.italic;
                    cell.underline = style.decoration == TextDecoration::Underline;
                    cell.strikethrough = style.decoration == TextDecoration::LineThrough;
                }
            }
        }
    }

    fn draw_outline(&mut self, x: f32, y: f32, w: f32, h: f32, color: [f32; 4]) {
        let x_int = x.round() as i32;
        let y_int = y.round() as i32;
        let w_int = w.round() as i32;
        let h_int = h.round() as i32;
        
        let term_w = self.core.logical_size.x.round() as i32;
        let term_h = self.core.logical_size.y.round() as i32;

        if w_int < 2 || h_int < 2 { return; }

        let set_border = |renderer: &mut TerminalRenderer, cx: i32, cy: i32, mask: u8| {
            if cx >= 0 && cx < term_w && cy >= 0 && cy < term_h {
                if renderer.is_clipped(cx, cy) { return; }
                let idx = (cy * term_w + cx) as usize;
                if idx < renderer.cells.len() {
                    renderer.cells[idx].border_mask |= mask;
                    renderer.cells[idx].fg = color;
                }
            }
        };

        // Corners (Combine existing logic with bitmasking)
        set_border(self, x_int, y_int, 2 | 8); // Down + Right
        set_border(self, x_int + w_int - 1, y_int, 2 | 4); // Down + Left
        set_border(self, x_int, y_int + h_int - 1, 1 | 8); // Up + Right
        set_border(self, x_int + w_int - 1, y_int + h_int - 1, 1 | 4); // Up + Left

        // Horizontals
        for i in 1..(w_int - 1) {
            set_border(self, x_int + i, y_int, 4 | 8); // Left + Right
            set_border(self, x_int + i, y_int + h_int - 1, 4 | 8);
        }

        // Verticals
        for j in 1..(h_int - 1) {
            set_border(self, x_int, y_int + j, 1 | 2); // Up + Down
            set_border(self, x_int + w_int - 1, y_int + j, 1 | 2);
        }
    }

    fn push_clip(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.clip_stack.push([x, y, w, h]);
    }

    fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    fn present(&mut self) {
        // --- BORDER RESOLUTION PHASE ---
        for cell in &mut self.cells {
            if cell.border_mask > 0 {
                cell.symbol = match cell.border_mask {
                    3 => "│", // Up + Down
                    12 => "─", // Left + Right
                    10 => "┌", // Down + Right
                    6 => "┐", // Down + Left
                    9 => "└", // Up + Right
                    5 => "┘", // Up + Left
                    11 => "├", // Up + Down + Right
                    7 => "┤", // Up + Down + Left
                    14 => "┬", // Down + Left + Right
                    13 => "┴", // Up + Left + Right
                    15 => "┼", // All
                    1 => "╵", // Up only
                    2 => "╷", // Down only
                    4 => "╴", // Left only
                    8 => "╶", // Right only
                    _ => &cell.symbol,
                }.to_string();
            }
        }

        let mut output = String::with_capacity(self.cells.len() * 16);
        let w = self.core.logical_size.x.round() as usize;
        let h = self.core.logical_size.y.round() as usize;

        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                if idx >= self.cells.len() || idx >= self.last_cells.len() { continue; }

                // Only draw if the cell has changed
                if self.cells[idx] != self.last_cells[idx] {
                    // Move Cursor
                    output.push_str(&format!("\x1B[{};{}H", y + 1, x + 1));
                    
                    let cell = &self.cells[idx];
                    
                    // Apply ANSI styles
                    output.push_str(&Self::color_to_ansi(cell.fg, false));
                    if cell.bg[3] > 0.0 {
                        output.push_str(&Self::color_to_ansi(cell.bg, true));
                    }
                    
                    if cell.bold { output.push_str("\x1B[1m"); }
                    if cell.italic { output.push_str("\x1B[3m"); }
                    if cell.underline { output.push_str("\x1B[4m"); }
                    if cell.strikethrough { output.push_str("\x1B[9m"); }
                    
                    output.push_str(&cell.symbol);
                    output.push_str("\x1B[0m"); // Reset for next potential skip
                }
            }
        }
        
        let _ = self.stdout.write_all(output.as_bytes());
        let _ = self.stdout.flush();
        
        // Sync buffers
        self.last_cells = self.cells.clone();
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
