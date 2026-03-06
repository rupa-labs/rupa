#[derive(Clone, Debug, PartialEq, Default)]
pub struct Transform {
    pub translate: [f32; 2],
    pub rotate: f32, // angle in degrees
    pub scale: [f32; 2],
    pub skew: [f32; 2],
}

impl Transform {
    pub fn new() -> Self {
        Self {
            translate: [0.0, 0.0],
            rotate: 0.0,
            scale: [1.0, 1.0],
            skew: [0.0, 0.0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_defaults() {
        let t = Transform::new();
        assert_eq!(t.scale, [1.0, 1.0]);
        assert_eq!(t.rotate, 0.0);
    }
}
