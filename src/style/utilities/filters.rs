use crate::style::utilities::effects::Shadow;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_stack() {
        let mut stack = FilterStack::default();
        stack.filters.push(Filter::Blur(5.0));
        stack.filters.push(Filter::Grayscale(1.0));
        
        assert_eq!(stack.filters.len(), 2);
        assert_eq!(stack.filters[0], Filter::Blur(5.0));
    }
}
