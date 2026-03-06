#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default)]
pub enum Breakpoint {
    #[default]
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Xl5,
    Xl6,
}

impl Breakpoint {
    pub fn min_width(&self) -> f32 {
        match self {
            Breakpoint::Xs => 0.0,
            Breakpoint::Sm => 640.0,
            Breakpoint::Md => 768.0,
            Breakpoint::Lg => 1024.0,
            Breakpoint::Xl => 1280.0,
            Breakpoint::Xl2 => 1536.0,
            Breakpoint::Xl3 => 1920.0,
            Breakpoint::Xl4 => 2560.0,
            Breakpoint::Xl5 => 3840.0,
            Breakpoint::Xl6 => 5120.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_order() {
        assert!(Breakpoint::Sm.min_width() > Breakpoint::Xs.min_width());
        assert!(Breakpoint::Md.min_width() > Breakpoint::Sm.min_width());
        assert!(Breakpoint::Xl6.min_width() > Breakpoint::Xl5.min_width());
    }
}
