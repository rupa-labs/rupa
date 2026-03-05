use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Signal, Variant};
use crate::Component;
use crate::container::Children;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FormState {
    #[default]
    Normal,
    Valid,
    Invalid,
}

/// A semantic component for text inputs and textareas.
pub struct Input {
    pub id: String,
    pub label: Option<String>,
    pub placeholder: String,
    pub value: Signal<String>,
    pub state: Signal<FormState>,
    pub is_textarea: bool,
    pub style: Style,
    pub accessibility: Accessibility,
}

impl Input {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            label: None,
            placeholder: placeholder.into(),
            value: Signal::new(String::new()),
            state: Signal::new(FormState::Normal),
            is_textarea: false,
            style: Style::default(),
            accessibility: Accessibility { role: Role::None, ..Default::default() },
        }
    }

    pub fn textarea(mut self) -> Self { self.is_textarea = true; self }
    pub fn label(mut self, text: impl Into<String>) -> Self { self.label = Some(text.into()); self }
    pub fn value(self, signal: Signal<String>) -> Self { let mut s = self; s.value = signal; s }
    pub fn state(self, signal: Signal<FormState>) -> Self { let mut s = self; s.state = signal; s }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }
}

impl Component for Input {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Input [{}] value='{}' state={:?}", self.id, self.value.get(), self.state.get());
    }
}

/// A semantic component for dropdown selections.
pub struct Select {
    pub id: String,
    pub options: Vec<(String, String)>, // (value, label)
    pub selected: Signal<String>,
    pub style: Style,
}

impl Select {
    pub fn new(options: Vec<(String, String)>) -> Self {
        Self {
            id: generate_id(),
            options,
            selected: Signal::new(String::new()),
            style: Style::default(),
        }
    }

    pub fn selected(self, signal: Signal<String>) -> Self { let mut s = self; s.selected = signal; s }
}

impl Component for Select {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Select [{}] selected='{}'", self.id, self.selected.get());
    }
}

/// A semantic component for Checkboxes and Radios.
pub struct Check {
    pub id: String,
    pub label: String,
    pub checked: Signal<bool>,
    pub is_radio: bool,
    pub style: Style,
}

impl Check {
    pub fn checkbox(label: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            label: label.into(),
            checked: Signal::new(false),
            is_radio: false,
            style: Style::default(),
        }
    }

    pub fn radio(label: impl Into<String>) -> Self {
        let mut s = Self::checkbox(label);
        s.is_radio = true;
        s
    }

    pub fn checked(self, signal: Signal<bool>) -> Self { let mut s = self; s.checked = signal; s }
}

impl Component for Check {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Check [{}] checked={} is_radio={}", self.id, self.checked.get(), self.is_radio);
    }
}

/// A semantic component for Range inputs.
pub struct Range {
    pub id: String,
    pub min: f32,
    pub max: f32,
    pub value: Signal<f32>,
    pub style: Style,
}

impl Range {
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            id: generate_id(),
            min,
            max,
            value: Signal::new(min),
            style: Style::default(),
        }
    }
}

impl Component for Range {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Range [{}] val={}", self.id, self.value.get());
    }
}
