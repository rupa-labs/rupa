#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Scale {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Xl5,
    Xl6,
}

impl Scale {
    /// Returns the multiplier for the scale based on a base value.
    pub fn multiplier(&self) -> f32 {
        match self {
            Scale::Xs => 0.75,
            Scale::Sm => 0.875,
            Scale::Md => 1.0,
            Scale::Lg => 1.125,
            Scale::Xl => 1.25,
            Scale::Xl2 => 1.5,
            Scale::Xl3 => 1.875,
            Scale::Xl4 => 2.25,
            Scale::Xl5 => 3.0,
            Scale::Xl6 => 3.75,
        }
    }

    /// Common pixel values for Spacing/Sizing based on a 4px (1rem = 16px) base.
    pub fn value(&self, base: f32) -> f32 {
        base * self.multiplier()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_scale_logic() {
        assert!(Scale::Xs.multiplier() < Scale::Md.multiplier());
        assert!(Scale::Xl6.multiplier() > Scale::Xl.multiplier());
        assert_eq!(Scale::Md.multiplier(), 1.0);
    }

    #[test]
    fn test_scale_values() {
        let base = 16.0;
        assert_eq!(Scale::Md.value(base), 16.0);
        assert_eq!(Scale::Xl2.value(base), 24.0); // 1.5 * 16
    }
}
