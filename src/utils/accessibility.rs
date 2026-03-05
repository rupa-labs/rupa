#[derive(Clone, Debug, PartialEq, Default)]
pub enum ForcedColorAdjust { #[default] Auto, None }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Accessibility {
    pub forced_color_adjust: ForcedColorAdjust,
}
