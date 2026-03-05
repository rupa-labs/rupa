#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default)]
pub enum Breakpoint {
    #[default]
    Xs, // < 576px
    Sm, // >= 576px
    Md, // >= 768px
    Lg, // >= 992px
    Xl, // >= 1200px
    Xxl, // >= 1400px
}

impl Breakpoint {
    pub fn min_width(&self) -> f32 {
        match self {
            Breakpoint::Xs => 0.0,
            Breakpoint::Sm => 576.0,
            Breakpoint::Md => 768.0,
            Breakpoint::Lg => 992.0,
            Breakpoint::Xl => 1200.0,
            Breakpoint::Xxl => 1400.0,
        }
    }
}
