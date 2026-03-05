use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Breakpoint, Theme, ColorMode};
use crate::Component;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BrandVisibility {
    #[default]
    Both,
    LogoOnly,
    TextOnly,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct BrandLogo {
    pub default: String,
    pub light: Option<String>,
    pub dark: Option<String>,
}

/// A semantic component for application branding (Logo + Name).
/// Supports theme-aware logos (switching between light/dark variants).
pub struct Brand {
    pub id: String,
    pub name: String,
    pub logo: Option<BrandLogo>,
    pub visibility: BrandVisibility,
    pub collapse_at: Breakpoint,
    pub style: Style,
    pub accessibility: Accessibility,
}

impl Brand {
    pub fn new(name: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);

        Self {
            id: generate_id(),
            name: name.into(),
            logo: None,
            visibility: BrandVisibility::Both,
            collapse_at: Breakpoint::Sm,
            style,
            accessibility: Accessibility { 
                role: Role::Link, 
                label: Some("Go to home".into()),
                ..Default::default() 
            },
        }
    }

    /// Set logos for different theme modes.
    /// - `default`: The fallback logo.
    /// - `light`: Optional logo for light mode.
    /// - `dark`: Optional logo for dark mode.
    pub fn logo(
        mut self, 
        default: impl Into<String>, 
        light: Option<String>, 
        dark: Option<String>
    ) -> Self {
        self.logo = Some(BrandLogo {
            default: default.into(),
            light,
            dark,
        });
        self
    }

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
        let theme = Theme::current();
        let active_logo = if let Some(ref logo_config) = self.logo {
            match theme.mode {
                ColorMode::Light => logo_config.light.as_ref().unwrap_or(&logo_config.default),
                ColorMode::Dark => logo_config.dark.as_ref().unwrap_or(&logo_config.default),
                ColorMode::System => &logo_config.default, // Simplified for now
            }
        } else {
            "none"
        };

        log::debug!(
            "Rendering Brand [{}] '{}' using logo: {} (Theme: {:?})", 
            self.id, self.name, active_logo, theme.mode
        );
    }
}
