use rupa_core::component::Component;

/// High-level utilities for inspecting and manipulating the Component Tree.
pub struct ElementTree;

impl ElementTree {
    /// Recursively find a component by its unique ID.
    pub fn find<'a>(root: &'a dyn Component, id: &str) -> Option<&'a dyn Component> {
        if root.id() == id {
            return Some(root);
        }

        for child in root.children() {
            if let Some(found) = Self::find(child, id) {
                return Some(found);
            }
        }

        None
    }

    /// Traverse the entire tree and execute a closure for every component.
    /// Useful for debugging, layout inspection, or accessibility tree generation.
    pub fn walk<F>(root: &dyn Component, f: &mut F) 
    where 
        F: FnMut(&dyn Component) 
    {
        f(root);
        for child in root.children() {
            Self::walk(child, f);
        }
    }

    /// Returns the total count of components in the tree.
    pub fn count(root: &dyn Component) -> usize {
        let mut total = 0;
        Self::walk(root, &mut |_| total += 1);
        total
    }

    /// Prints a visual representation of the tree to the log.
    pub fn debug_dump(root: &dyn Component, depth: usize) {
        let indent = "  ".repeat(depth);
        log::debug!("{}[{}] {}", indent, root.id(), std::any::type_name::<Self>());
        for child in root.children() {
            Self::debug_dump(child, depth + 1);
        }
    }
}
