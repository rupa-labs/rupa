use crate::utils::{Style, generate_id, Accessibility, Role, Variant};
use crate::Component;
use crate::container::Children;

/// A top-level navigation bar.
pub struct Navbar {
    pub id: String,
    pub brand: Option<String>,
    pub style: Style,
    pub children: Children,
}

impl Navbar {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            brand: None,
            style: Style::default(),
            children: Children::new(),
        }
    }

    pub fn brand(mut self, name: impl Into<String>) -> Self { self.brand = Some(name.into()); self }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}

impl Component for Navbar {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Navbar [{}] brand={:?}", self.id, self.brand);
        self.children.render_all();
    }
}

/// A list of navigation links.
pub struct Nav {
    pub id: String,
    pub style: Style,
    pub children: Children,
}

impl Nav {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            style: Style::default(),
            children: Children::new(),
        }
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}

impl Component for Nav {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Nav [{}]", self.id);
        self.children.render_all();
    }
}

/// A semantic component for paginated content.
pub struct Pagination {
    pub id: String,
    pub style: Style,
}

impl Pagination {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            style: Style::default(),
        }
    }
}

impl Component for Pagination {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Pagination [{}]", self.id);
    }
}

// ... Keep existing Breadcrumb code ...
pub struct Breadcrumb {
    pub id: String,
    pub items: Vec<(String, Option<String>)>, // (label, url)
    pub style: Style,
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            items: Vec::new(),
            style: Style::default(),
        }
    }
    pub fn item(mut self, label: impl Into<String>, url: Option<String>) -> Self {
        self.items.push((label.into(), url));
        self
    }
}

impl Component for Breadcrumb {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Breadcrumb [{}] with {} items", self.id, self.items.len());
    }
}
