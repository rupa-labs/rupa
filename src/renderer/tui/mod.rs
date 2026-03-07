use crate::support::vector::Vec2;
use crate::style::utilities::typography::TextAlign;
use crate::renderer::{Renderer, RenderCore};
use std::io::{Write, stdout};

#[derive(Clone, Copy, PartialEq)]
struct TuiCell {
    symbol: char,
    fg: [u8; 3],
    bg: [u8; 3],
}

impl Default for TuiCell {
    fn default() -> Self {
        Self { symbol: ' ', fg: [255, 255, 255], bg: [0, 0, 0] }
    }
}

pub struct TuiRenderer {
    pub core: RenderCore,
    buffer: Vec<TuiCell>,
    prev_buffer: Vec<TuiCell>,
}

impl TuiRenderer {
    pub fn new(width: u16, height: u16) -> Self {
        let size = (width * height) as usize;
        Self {
            core: RenderCore::new(width as f32, height as f32, 1.0),
            buffer: vec![TuiCell::default(); size],
            prev_buffer: vec![TuiCell::default(); size],
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.core.logical_size = Vec2::new(width as f32, height as f32);
        self.core.scale_factor = 1.0;
        let size = (width * height) as usize;
        self.buffer = vec![TuiCell::default(); size];
        self.prev_buffer = vec![TuiCell::default(); size];
    }

    pub fn width(&self) -> u16 { self.core.logical_size.x as u16 }
    pub fn height(&self) -> u16 { self.core.logical_size.y as u16 }

    fn put_char(&mut self, x: i32, y: i32, c: char, fg: [u8; 3], bg: [u8; 3]) {
        let w = self.width() as i32;
        let h = self.height() as i32;
        if x < 0 || x >= w || y < 0 || y >= h {
            return;
        }
        let idx = (y * w + x) as usize;
        self.buffer[idx] = TuiCell { symbol: c, fg, bg };
    }
}

impl crate::renderer::TextMeasurer for TuiRenderer {
    fn measure(&self, text: &str, _size: f32) -> Vec2 {
        let width = text.chars().count() as f32;
        let height = 1.0; // TUI text is usually single line unless wrapped
        Vec2::new(width, height)
    }
}

impl Renderer for TuiRenderer {
    fn core(&self) -> &RenderCore { &self.core }
    fn core_mut(&mut self) -> &mut RenderCore { &mut self.core }

    fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, color: [f32; 4], _radius: f32) {
        let scale = self.core.scale_factor;
        let tx = (x + self.core.camera_offset.x) * self.core.camera_zoom * scale;
        let ty = (y + self.core.camera_offset.y) * self.core.camera_zoom * scale;
        let tw = width * self.core.camera_zoom * scale;
        let th = height * self.core.camera_zoom * scale;

        let ix = tx.round() as i32;
        let iy = ty.round() as i32;
        let iw = tw.round() as i32;
        let ih = th.round() as i32;

        let fg = [200, 200, 200];
        let bg = [(color[0] * 255.0) as u8, (color[1] * 255.0) as u8, (color[2] * 255.0) as u8];

        for row in 0..ih {
            for col in 0..iw {
                let mut symbol = ' ';
                if row == 0 && col == 0 { symbol = '┌'; }
                else if row == 0 && col == iw - 1 { symbol = '┐'; }
                else if row == ih - 1 && col == 0 { symbol = '└'; }
                else if row == ih - 1 && col == iw - 1 { symbol = '┘'; }
                else if row == 0 || row == ih - 1 { symbol = '─'; }
                else if col == 0 || col == iw - 1 { symbol = '│'; }
                
                self.put_char(ix + col, iy + row, symbol, fg, bg);
            }
        }
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32, _size: f32, color: [f32; 4], _align: TextAlign) {
        let scale = self.core.scale_factor;
        let tx = (x + self.core.camera_offset.x) * self.core.camera_zoom * scale;
        let ty = (y + self.core.camera_offset.y) * self.core.camera_zoom * scale;
        let ix = tx.round() as i32;
        let iy = ty.round() as i32;
        let fg = [(color[0] * 255.0) as u8, (color[1] * 255.0) as u8, (color[2] * 255.0) as u8];
        let bg = [0, 0, 0];
        for (i, c) in text.chars().enumerate() {
            self.put_char(ix + i as i32, iy, c, fg, bg);
        }
    }

    fn push_clip(&mut self, _x: f32, _y: f32, _w: f32, _h: f32) {}
    fn pop_clip(&mut self) {}

    fn present(&mut self) {
        let mut out = stdout().lock();
        let w = self.width() as usize;
        let h = self.height() as usize;
        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let cell = self.buffer[idx];
                if cell != self.prev_buffer[idx] {
                    write!(out, "\x1b[{};{}H\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m{}", 
                        y + 1, x + 1, 
                        cell.fg[0], cell.fg[1], cell.fg[2],
                        cell.bg[0], cell.bg[1], cell.bg[2],
                        cell.symbol).unwrap();
                }
            }
        }
        out.flush().unwrap();
        self.prev_buffer.copy_from_slice(&self.buffer);
        self.buffer.fill(TuiCell::default());
    }
}
