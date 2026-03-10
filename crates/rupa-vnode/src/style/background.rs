use serde::{Serialize, Deserialize};
use crate::color::Color;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Background {
    pub color: Option<Color>,
    pub image: Option<String>,
}
