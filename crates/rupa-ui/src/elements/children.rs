use rupa_core::Component;
use rupa_vnode::VNode;

pub struct Children<'a> {
    pub items: Vec<Box<dyn Component + 'a>>,
}

impl<'a> Children<'a> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, child: Box<dyn Component + 'a>) {
        self.items.push(child);
    }

    pub fn add(&mut self, child: Box<dyn Component + 'a>) {
        self.items.push(child);
    }

    pub fn as_refs(&self) -> Vec<&dyn Component> {
        self.items.iter().map(|c| c.as_ref()).collect()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Box<dyn Component + 'a>> {
        self.items.iter()
    }

    pub fn render_all(&self) -> Vec<VNode> {
        self.items.iter().map(|c| c.render()).collect()
    }
}
