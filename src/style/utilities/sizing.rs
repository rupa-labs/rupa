#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sizing {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub min_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_width: Option<f32>,
    pub max_height: Option<f32>,
    pub aspect_ratio: Option<f32>,
}
