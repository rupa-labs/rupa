use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Breakpoint};
use crate::Component;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BrandVisibility {
    #[default]
    Both,      // Show logo and text
    LogoOnly,  // Hide text on certain breakpoints
    TextOnly,
}

/// A semantic component for application branding (Logo + Name).
/// Built-in responsiveness: can automatically collapse to LogoOnly on small screens.
pub struct Brand {
    pub id: String,
    pub name: String,
    pub logo_url: Option<String>,
    pub visibility: BrandVisibility,
    pub collapse_at: Breakpoint, // Threshold to switch to LogoOnly
    pub style: Style,
    pub accessibility: Accessibility,
}

impl Brand {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            name: name.into(),
            logo_url: None,
            visibility: BrandVisibility::Both,
            collapse_at: Breakpoint::Sm,
            style: Style::default(),
            accessibility: Accessibility { 
                role: Role::Link, 
                label: Some("Go to home".into()),
                ..Default::default() 
            },
        }
    }

    pub fn logo(mut self, url: impl Into<String>) -> Self {
        self.logo_url = Some(url.into());
        self
    }

    /// Set when the brand should switch to Logo-only mode.
    pub fn collapse_at(mut self, bp: Breakpoint) -> Self {
        self.collapse_at = bp;
        self
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }
}

impl Component for Brand {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!(
            "Rendering Brand [{}] '{}' (Logo: {:?}) - Collapses at: {:?}", 
            self.id, self.name, self.logo_url, self.collapse_at
        );
        // Logic: If current_viewport_width < collapse_at.min_width(), 
        // only render the logo.
    }
}
