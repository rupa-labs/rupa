use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role};
use crate::Component;
use crate::container::Children;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ContainerVariant {
    #[default]
    Fixed,
    Fluid,
}

/// A semantic component that centers and limits content width.
pub struct Container {
    pub id: String,
    pub variant: ContainerVariant,
    pub style: Style,
    pub accessibility: Accessibility,
    pub children: Children,
}

impl Container {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            variant: ContainerVariant::Fixed,
            style: Style::default(),
            accessibility: Accessibility { role: Role::None, ..Default::default() },
            children: Children::new(),
        }
    }

    pub fn fluid(mut self) -> Self { self.variant = ContainerVariant::Fluid; self }
    
    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.add(child);
        self
    }
}

impl Component for Container {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Container [{}]: {:?}", self.id, self.variant);
        self.children.render_all();
    }
}

/// A semantic component for Grid Rows.
pub struct Row {
    pub id: String,
    pub style: Style,
    pub children: Children,
}

impl Row {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.display = crate::utils::Display::Flex;
        style.flex.direction = crate::utils::FlexDirection::Row;
        style.flex.wrap = crate::utils::FlexWrap::Wrap;
        
        Self {
            id: generate_id(),
            style,
            children: Children::new(),
        }
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }

    pub fn gutter(mut self, val: f32) -> Self {
        self.style.grid.gap = (val, val);
        self
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.add(child);
        self
    }
}

impl Component for Row {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Row [{}]", self.id);
        self.children.render_all();
    }
}

/// A semantic component for Grid Columns.
pub struct Col {
    pub id: String,
    pub span: u16, // 1-12
    pub style: Style,
    pub children: Children,
}

impl Col {
    pub fn new(span: u16) -> Self {
        Self {
            id: generate_id(),
            span,
            style: Style::default(),
            children: Children::new(),
        }
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.add(child);
        self
    }
}

impl Component for Col {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Col [{}] span={}", self.id, self.span);
        self.children.render_all();
    }
}
