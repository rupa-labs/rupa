use rupa_base::{Color, Vec2, Rect};
use serde::{Serialize, Deserialize};

/// Low-level drawing commands for the Canvas Port.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    MoveTo(Vec2),
    LineTo(Vec2),
    Rect(Rect, Color),
    Circle(Vec2, f32, Color),
    Text(String, Vec2, f32, Color),
    Clip(Rect),
    PopClip,
}

/// A reactive canvas state that accumulates drawing commands.
pub struct Canvas {
    commands: Vec<Command>,
}

impl Canvas {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn draw_rect(&mut self, rect: Rect, color: Color) {
        self.commands.push(Command::Rect(rect, color));
    }

    pub fn draw_text(&mut self, text: impl Into<String>, pos: Vec2, size: f32, color: Color) {
        self.commands.push(Command::Text(text.into(), pos, size, color));
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }
}
