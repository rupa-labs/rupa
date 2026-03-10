use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Display {
    #[default]
    Flex,
    Grid,
    None,
    Block,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Position {
    #[default]
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Layout {
    pub display: Display,
    pub position: Position,
    pub z_index: i32,
}
