use rupa_core::VNode;
use rupa_core::scene::layout::{LayoutSolver, TaffySolver};
use rupa_core::renderer::TextMeasurer;
use rupa_core::scene::SceneNode;

pub struct TerminalLayout;

impl TerminalLayout {
    /// Builds and computes the layout for a VNode tree in Cell mode.
    pub fn compute(
        node: &VNode, 
        measurer: &dyn TextMeasurer, 
        width: f32, 
        height: f32
    ) -> SceneNode {
        let mut solver = TaffySolver::new();
        // The solver itself now handles VNode -> Taffy mapping internally via LayoutSolver trait
        // but since TaffySolver::solve is public, we can use it directly.
        solver.solve(node, measurer, width, height)
    }
}
