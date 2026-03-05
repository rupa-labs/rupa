#[derive(Clone, Debug, PartialEq, Default)]
pub struct Sizing {
    // Physical Dimensions
    pub width: Option<f32>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub height: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,

    // Logical Dimensions (Writing-mode aware)
    pub inline_size: Option<f32>,
    pub min_inline_size: Option<f32>,
    pub max_inline_size: Option<f32>,
    pub block_size: Option<f32>,
    pub min_block_size: Option<f32>,
    pub max_block_size: Option<f32>,
}
