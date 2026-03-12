use rupa_core::{Vec2, Error, Renderer, vnode::VNode, reconciler::reconcile};
use rupa_core::events::InputEvent;
use rupa_core::events::dispatcher::InputDispatcher;
use rupa_tui::TerminalRenderer;
use rupa_engine::platform::{SharedPlatformCore, runner::*, register_redraw_proxy, AppMetadata};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, size, EnterAlternateScreen, LeaveAlternateScreen},
    cursor::{Hide, Show},
    ExecutableCommand,
};
use std::time::Duration;
use taffy::prelude::*;

pub struct TerminalRunner {
    pub core: SharedPlatformCore,
    pub renderer: TerminalRenderer,
    pub last_vnode: VNode,
}

impl TerminalRunner {
    pub fn new(core: SharedPlatformCore) -> Self {
        let (w, h) = size().unwrap_or((80, 24));
        Self {
            core,
            renderer: TerminalRenderer::new(w as f32, h as f32),
            last_vnode: VNode::Empty,
        }
    }

    fn handle_redraw(&mut self) {
        let mut core = match self.core.write() {
            Ok(c) => c,
            Err(_) => return,
        };

        if let Some(root) = core.root.take() {
            let (w, h) = size().unwrap_or((80, 24));
            
            // 1. Render Current Tree
            let new_vnode = root.render();

            // 2. Reconcile (Identify Changes)
            let patches = reconcile(&self.last_vnode, &new_vnode, None, 0);
            
            // 3. Update Taffy Tree (For Layout & Hit Testing)
            // Note: In a matured engine, the reconciler generates patches that 
            // the renderer applies to its internal scene graph. 
            // For this terminal showroom, we rebuild layout when tree structure changes.
            core.scene.layout_engine.taffy.clear();
            let layout_root = self.build_taffy_from_vnode(&new_vnode, &mut core.scene.layout_engine.taffy);
            
            core.scene.layout_engine.taffy.compute_layout(
                layout_root, 
                Size { width: AvailableSpace::Definite(w as f32), height: AvailableSpace::Definite(h as f32) }
            ).unwrap();

            // 4. Paint the VNode tree
            self.renderer.clear_screen();
            Self::paint_vnode(&mut self.renderer, &new_vnode, &core.scene.layout_engine.taffy, layout_root, Vec2::zero());
            self.renderer.present();

            self.last_vnode = new_vnode;
            core.root = Some(root);
        }
    }

    fn paint_vnode(
        renderer: &mut TerminalRenderer,
        node: &VNode,
        taffy: &TaffyTree<()>,
        layout_node: NodeId,
        global_pos: Vec2,
    ) {
        let layout = taffy.layout(layout_node).unwrap();
        let pos = global_pos + Vec2::new(layout.location.x, layout.location.y);

        match node {
            VNode::Element(el) => {
                // Draw Background
                if let Some(ref color) = el.style.background.color {
                    let rgba: [f32; 4] = color.to_rgba();
                    renderer.draw_rect(pos.x, pos.y, layout.size.width, layout.size.height, rgba, 0.0);
                }

                if el.style.border.width != 0.0 {
                    renderer.draw_outline(pos.x, pos.y, layout.size.width, layout.size.height, [0.5, 0.5, 0.5, 1.0]);
                }

                let taffy_children = taffy.children(layout_node).unwrap();
                for (i, child) in el.children.iter().enumerate() {
                    if let Some(child_layout_node) = taffy_children.get(i) {
                        Self::paint_vnode(renderer, child, taffy, *child_layout_node, pos);
                    }
                }
            }
            VNode::Text(text) => {
                let color = [1.0, 1.0, 1.0, 1.0];
                renderer.draw_text(text, pos.x, pos.y, layout.size.width, 1.0, color, rupa_core::vnode::TextAlign::Left);
            }
            VNode::Fragment(children) => {
                let taffy_children = taffy.children(layout_node).unwrap();
                for (i, child) in children.iter().enumerate() {
                    if let Some(child_layout_node) = taffy_children.get(i) {
                        Self::paint_vnode(renderer, child, taffy, *child_layout_node, pos);
                    }
                }
            }
            _ => {}
        }
    }

    fn build_taffy_from_vnode(&self, node: &VNode, taffy: &mut TaffyTree<()>) -> NodeId {
        match node {
            VNode::Element(el) => {
                let mut style = el.style.to_taffy();
                if style.size.width == Dimension::Auto { style.size.width = Dimension::Percent(1.0); }
                
                let children: Vec<NodeId> = el.children.iter()
                    .map(|c| self.build_taffy_from_vnode(c, taffy))
                    .collect();
                
                taffy.new_with_children(style, &children).unwrap()
            }
            VNode::Text(text) => {
                let mut style = taffy::prelude::Style::default();
                style.size.width = Dimension::Length(text.len() as f32);
                style.size.height = Dimension::Length(1.0);
                taffy.new_leaf(style).unwrap()
            }
            VNode::Fragment(children) => {
                let mut style = taffy::prelude::Style::default();
                style.display = Display::Flex;
                let children: Vec<NodeId> = children.iter()
                    .map(|c| self.build_taffy_from_vnode(c, taffy))
                    .collect();
                taffy.new_with_children(style, &children).unwrap()
            }
            _ => taffy.new_leaf(taffy::prelude::Style::default()).unwrap(),
        }
    }

    fn dispatch_event(&mut self, event: InputEvent) {
        let mut core = match self.core.write() {
            Ok(c) => c,
            Err(_) => return,
        };
        
        let mut cursor_pos = core.cursor_pos;
        let mut requested_cursor = core.requested_cursor;
        let mut pointer_capture = core.pointer_capture.take();
        let mut focused_id = core.focused_id.take();
        let mut hovered_path = std::mem::take(&mut core.hovered_path);
        let event_listeners = core.event_listeners.clone();
        
        InputDispatcher::dispatch(
            event,
            &self.last_vnode,
            &core.scene,
            &core.viewport,
            &mut cursor_pos,
            &mut requested_cursor,
            &mut pointer_capture,
            &mut focused_id,
            &mut hovered_path,
            &event_listeners,
            core.debug,
        );

        core.cursor_pos = cursor_pos;
        core.requested_cursor = requested_cursor;
        core.pointer_capture = pointer_capture;
        core.focused_id = focused_id;
        core.hovered_path = hovered_path;
    }
}

impl PlatformRunner for TerminalRunner {
    fn sync_metadata(&self, _metadata: &AppMetadata) -> Result<(), Error> {
        Ok(())
    }

    fn run(mut self) -> Result<(), Error> {
        let mut out = std::io::stdout();
        enable_raw_mode().map_err(|e| Error::Platform(format!("Failed to enable raw mode: {}", e)))?;
        out.execute(EnterAlternateScreen).unwrap();
        out.execute(Hide).unwrap();
        
        register_redraw_proxy(Box::new(|| {}));

        loop {
            if rupa_motion::GLOBAL_TIMELINE.tick() { }
            self.handle_redraw();

            if event::poll(Duration::from_millis(32)).unwrap_or(false) {
                match event::read().unwrap() {
                    Event::Key(key) => {
                        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                            break;
                        }
                        
                        let rupa_key = match key.code {
                            KeyCode::Enter => rupa_core::events::KeyCode::Enter,
                            KeyCode::Esc => rupa_core::events::KeyCode::Escape,
                            KeyCode::Up => rupa_core::events::KeyCode::ArrowUp,
                            KeyCode::Down => rupa_core::events::KeyCode::ArrowDown,
                            KeyCode::Left => rupa_core::events::KeyCode::ArrowLeft,
                            KeyCode::Right => rupa_core::events::KeyCode::ArrowRight,
                            KeyCode::Char(c) => rupa_core::events::KeyCode::Char(c),
                            KeyCode::Tab => rupa_core::events::KeyCode::Tab,
                            KeyCode::Backspace => rupa_core::events::KeyCode::Backspace,
                            _ => rupa_core::events::KeyCode::Unknown,
                        };

                        let state = rupa_core::events::ButtonState::Pressed;
                        let modifiers = rupa_core::events::Modifiers {
                            shift: key.modifiers.contains(KeyModifiers::SHIFT),
                            ctrl: key.modifiers.contains(KeyModifiers::CONTROL),
                            alt: key.modifiers.contains(KeyModifiers::ALT),
                            logo: false,
                        };

                        self.dispatch_event(InputEvent::Key { 
                            key: rupa_key, 
                            state, 
                            modifiers 
                        });
                    }
                    Event::Resize(w, h) => {
                        self.renderer.core.logical_size = Vec2::new(w as f32, h as f32);
                    }
                    _ => {}
                }
            }
        }

        out.execute(Show).unwrap();
        out.execute(LeaveAlternateScreen).unwrap();
        disable_raw_mode().unwrap();
        Ok(())
    }
}
