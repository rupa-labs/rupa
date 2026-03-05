use crate::utils::effects::Shadow;

#[derive(Clone, Debug, PartialEq)]
pub enum Filter {
    Blur(f32),
    Brightness(f32),
    Contrast(f32),
    DropShadow(Shadow),
    Grayscale(f32),
    HueRotate(f32),
    Invert(f32),
    Saturate(f32),
    Sepia(f32),
    Opacity(f32),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FilterStack {
    pub filters: Vec<Filter>,
    pub backdrop: Vec<Filter>,
}
