use rupa_core::{Component, VNode, ViewCore, Id, Signal};
use std::sync::Arc;

// --- SHOW ---

pub struct ShowLogic {
    pub condition: Signal<bool>,
    pub fallback: VNode,
}

pub struct ShowView {
    pub core: Arc<ViewCore>,
}

pub struct Show<'a> {
    pub id: String,
    pub logic: ShowLogic,
    pub view: ShowView,
    pub child: Box<dyn Component + 'a>,
}

impl<'a> Show<'a> {
    pub fn new(condition: Signal<bool>, child: impl Component + 'a) -> Self {
        Self {
            id: Id::next().to_string(),
            logic: ShowLogic { condition, fallback: VNode::Empty },
            view: ShowView { core: Arc::new(ViewCore::new()) },
            child: Box::new(child),
        }
    }
}

impl<'a> Component for Show<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![self.child.as_ref()] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        if self.logic.condition.get() {
            self.child.render()
        } else {
            self.logic.fallback.clone()
        }
    }
}

// --- FOR EACH ---

pub struct ForEachLogic<T> {
    pub items: Signal<Vec<T>>,
}

pub struct ForEachView {
    pub core: Arc<ViewCore>,
}

pub struct ForEach<'a, T> {
    pub id: String,
    pub logic: ForEachLogic<T>,
    pub view: ForEachView,
    pub template: Arc<dyn Fn(&T) -> Box<dyn Component + 'a> + Send + Sync>,
}

impl<'a, T: 'static + Send + Sync + Clone> ForEach<'a, T> {
    pub fn new(items: Signal<Vec<T>>, template: impl Fn(&T) -> Box<dyn Component + 'a> + Send + Sync + 'static) -> Self {
        Self {
            id: Id::next().to_string(),
            logic: ForEachLogic { items },
            view: ForEachView { core: Arc::new(ViewCore::new()) },
            template: Arc::new(template),
        }
    }
}

impl<'a, T: 'static + Send + Sync + Clone> Component for ForEach<'a, T> {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.core.clone() }
    
    fn render(&self) -> VNode {
        let mut children = Vec::new();
        for item in self.logic.items.get().iter() {
            let comp = (self.template)(item);
            children.push(comp.render());
        }
        VNode::Fragment(children)
    }
}
