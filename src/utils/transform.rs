#[derive(Clone, Debug, PartialEq, Default)]
pub enum TransformStyle { #[default] Flat, Preserve3d }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BackfaceVisibility { #[default] Visible, Hidden }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Transform {
    pub translate: [f32; 3], // [x, y, z]
    pub rotate: [f32; 3],    // [x, y, z] in degrees
    pub scale: [f32; 3],     // [x, y, z]
    pub skew: [f32; 2],      // [x, y]
    pub origin: [f32; 3],    // [x, y, z] pivot point
    pub perspective: Option<f32>,
    pub perspective_origin: [f32; 2],
    pub style: TransformStyle,
    pub backface: BackfaceVisibility,
    pub raw_matrix: Option<[f32; 16]>,
}
