use rupa_core::component::{Component, ComponentPtr};

/// # Rupa Element Tree 🌳
/// 
/// The stateful orchestrator for the Virtual Component Tree. 
/// It manages the lifecycle of components (Mounting, Unmounting) and 
/// provides structural utilities for the Engine.
pub struct ElementTree {
    /// The root of the entire virtual application.
    root: Option<ComponentPtr>,
}

impl ElementTree {
    /// Creates a new, empty Element Tree.
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Sets the root component of the tree and triggers the [mount] lifecycle.
    pub fn set_root(&mut self, component: ComponentPtr) {
        // 1. If there's an existing root, unmount it first
        if let Some(old_root) = self.root.take() {
            self.unmount_recursive(old_root.as_ref());
        }

        // 2. Set the new root and trigger mount
        self.mount_recursive(component.as_ref());
        self.root = Some(component);
    }

    /// Returns a reference to the root component, if any.
    pub fn root(&self) -> Option<&ComponentPtr> {
        self.root.as_ref()
    }

    /// Recursively mounts a component and its logical children.
    fn mount_recursive(&self, component: &dyn Component) {
        component.mount();
        for child in component.children() {
            self.mount_recursive(child);
        }
    }

    /// Recursively unmounts a component and its logical children.
    fn unmount_recursive(&self, component: &dyn Component) {
        for child in component.children() {
            self.unmount_recursive(child);
        }
        component.unmount();
    }

    /// --- Structural Utilities ---

    /// Recursively find a component by its debug name.
    pub fn find_by_name<'a>(root: &'a dyn Component, name: &str) -> Option<&'a dyn Component> {
        if root.debug_name() == name {
            return Some(root);
        }

        for child in root.children() {
            if let Some(found) = Self::find_by_name(child, name) {
                return Some(found);
            }
        }

        None
    }

    /// Traverse the entire tree and execute a closure for every component.
    pub fn walk<F>(root: &dyn Component, f: &mut F) 
    where 
        F: FnMut(&dyn Component) 
    {
        f(root);
        for child in root.children() {
            Self::walk(child, f);
        }
    }

    /// Returns the total count of components currently alive in the tree.
    pub fn count(&self) -> usize {
        match &self.root {
            Some(root) => {
                let mut total = 0;
                Self::walk(root.as_ref(), &mut |_| total += 1);
                total
            }
            None => 0,
        }
    }

    /// Prints a visual representation of the Virtual Component Tree for debugging.
    pub fn debug_dump(&self) {
        if let Some(root) = &self.root {
            Self::dump_recursive(root.as_ref(), 0);
        } else {
            log::debug!("<Empty Element Tree>");
        }
    }

    fn dump_recursive(component: &dyn Component, depth: usize) {
        let indent = "  ".repeat(depth);
        log::debug!("{}[{}]", indent, component.debug_name());
        for child in component.children() {
            Self::dump_recursive(child, depth + 1);
        }
    }
}
