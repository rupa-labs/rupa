use serde::{Serialize, Deserialize};
use crate::effects::Shadow;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    Blur(f32),
    Brightness(f32),
    Contrast(f32),
    Grayscale(f32),
    HueRotate(f32),
    Invert(f32),
    Opacity(f32),
    Saturate(f32),
    Sepia(f32),
    DropShadow(Shadow),
}
