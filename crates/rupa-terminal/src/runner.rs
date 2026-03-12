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
use unicode_width::UnicodeWidthStr;

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
            let _patches = reconcile(&self.last_vnode, &new_vnode, None, 0);
            
            // 3. Update Taffy Tree (For Layout & Hit Testing)
            core.scene.layout_engine.taffy.clear();
            let layout_root = self.build_taffy_from_vnode(&new_vnode, &mut core.scene.layout_engine.taffy);
            
            core.scene.layout_engine.taffy.compute_layout(
                layout_root, 
                Size { width: AvailableSpace::Definite(w as f32), height: AvailableSpace::Definite(h as f32) }
            ).unwrap();

            // 4. Paint the VNode tree
            self.renderer.clear_screen();
            let focused_id = core.focused_id.clone();
            Self::paint_vnode(&mut self.renderer, &new_vnode, &core.scene.layout_engine.taffy, layout_root, Vec2::zero(), focused_id.as_deref(), None);
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
        focused_id: Option<&str>,
        inherited_color: Option<[f32; 4]>,
    ) {
        let layout = taffy.layout(layout_node).unwrap();
        let pos = global_pos + Vec2::new(layout.location.x, layout.location.y);
        
        let rx = pos.x.round();
        let ry = pos.y.round();
        let rw = layout.size.width.round();
        let rh = layout.size.height.round();

        match node {
            VNode::Element(el) => {
                let is_focused = el.key.as_deref() == focused_id && focused_id.is_some();
                let is_input = el.tag == "input";
                let is_button = el.tag == "button" || el.handlers.on_click.is_some();
                
                // Determine typography color
                let mut text_color = el.style.typography.color.as_ref().map(|c| c.to_rgba()).or(inherited_color);

                // Draw Background
                let mut color = el.style.background.color.as_ref().map(|c| c.to_rgba());
                
                // Artisan Focus Styling (Subtle & Semantic)
                if is_focused {
                    if is_input {
                        color = Some([0.05, 0.05, 0.1, 1.0]); // Very deep dark
                    } else if is_button {
                        // Subtle highlight instead of solid blue
                        color = Some([0.1, 0.3, 0.5, 1.0]); 
                        text_color = Some([1.0, 1.0, 1.0, 1.0]);
                    }
                }

                if let Some(rgba) = color {
                    renderer.draw_rect(rx, ry, rw, rh, rgba, 0.0);
                }

                // Border logic
                if el.style.border.width != 0.0 || is_focused {
                    let border_color = if is_focused { 
                        if is_input { [1.0, 1.0, 0.0, 1.0] } else { [0.0, 0.8, 1.0, 1.0] } 
                    } else { 
                        [0.2, 0.2, 0.2, 1.0] 
                    };
                    renderer.draw_outline(rx, ry, rw, rh, border_color);
                }

                // Focus Indicators
                if is_focused {
                    if is_input {
                        renderer.draw_text("┃", rx - 1.0, ry, 1.0, 1.0, [1.0, 1.0, 0.0, 1.0], rupa_core::vnode::TextAlign::Left);
                    } else if is_button {
                        renderer.draw_text(">", rx - 2.0, ry, 1.0, 1.0, [0.0, 1.0, 1.0, 1.0], rupa_core::vnode::TextAlign::Left);
                    }
                }

                // Render Input Value with Cursor
                if is_input {
                    let val = el.attributes.get("value").cloned().unwrap_or_default();
                    let display_text = if is_focused { format!("{}_", val) } else { val };
                    renderer.draw_text(&display_text, rx + 1.0, ry, rw - 2.0, 1.0, [1.0, 1.0, 1.0, 1.0], rupa_core::vnode::TextAlign::Left);
                }

                let taffy_children = taffy.children(layout_node).unwrap();
                for (i, child) in el.children.iter().enumerate() {
                    if let Some(child_layout_node) = taffy_children.get(i) {
                        Self::paint_vnode(renderer, child, taffy, *child_layout_node, pos, focused_id, text_color);
                    }
                }
            }
            VNode::Text(text) => {
                let color = inherited_color.unwrap_or([0.9, 0.9, 0.9, 1.0]);
                renderer.draw_text(text, rx, ry, rw, 1.0, color, rupa_core::vnode::TextAlign::Left);
            }
            VNode::Fragment(children) => {
                let taffy_children = taffy.children(layout_node).unwrap();
                for (i, child) in children.iter().enumerate() {
                    if let Some(child_layout_node) = taffy_children.get(i) {
                        Self::paint_vnode(renderer, child, taffy, *child_layout_node, pos, focused_id, inherited_color);
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
                if el.tag == "input" {
                    style.size.height = Dimension::Length(1.0);
                    style.padding = taffy::prelude::Rect {
                        left: length(1.0), right: length(1.0), top: length(0.0), bottom: length(0.0)
                    };
                }
                
                let children: Vec<NodeId> = el.children.iter()
                    .map(|c| self.build_taffy_from_vnode(c, taffy))
                    .collect();
                
                taffy.new_with_children(style, &children).unwrap()
            }
            VNode::Text(text) => {
                let mut style = taffy::prelude::Style::default();
                style.size.width = Dimension::Length(text.width() as f32);
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

        // TUI Focus Management
        if let InputEvent::Key { key, state, .. } = &event {
            if *state == rupa_core::events::ButtonState::Pressed {
                let clickable_ids = Self::find_clickable_ids(&self.last_vnode);
                if !clickable_ids.is_empty() {
                    let mut current_idx = focused_id.as_ref()
                        .and_then(|id| clickable_ids.iter().position(|c| c == id))
                        .unwrap_or(0);

                    match key {
                        rupa_core::events::KeyCode::ArrowDown | rupa_core::events::KeyCode::Tab => {
                            current_idx = (current_idx + 1) % clickable_ids.len();
                            focused_id = Some(clickable_ids[current_idx].clone());
                            core.focused_id = focused_id.clone();
                            return;
                        }
                        rupa_core::events::KeyCode::ArrowUp => {
                            current_idx = (current_idx + clickable_ids.len() - 1) % clickable_ids.len();
                            focused_id = Some(clickable_ids[current_idx].clone());
                            core.focused_id = focused_id.clone();
                            return;
                        }
                        _ => {}
                    }
                }
            }
        }
        
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

    fn find_clickable_ids(node: &VNode) -> Vec<String> {
        let mut ids = Vec::new();
        match node {
            VNode::Element(el) => {
                // Focusable if it has handlers or is an input
                if el.handlers.on_click.is_some() || el.handlers.on_input.is_some() || el.tag == "input" {
                    if let Some(ref key) = el.key {
                        ids.push(key.clone());
                    }
                }
                for child in &el.children {
                    ids.extend(Self::find_clickable_ids(child));
                }
            }
            VNode::Fragment(children) => {
                for child in children {
                    ids.extend(Self::find_clickable_ids(child));
                }
            }
            _ => {}
        }
        ids
    }
}

impl PlatformRunner for TerminalRunner {
    fn sync_metadata(&self, _metadata: &AppMetadata) -> Result<(), Error> {
        Ok(())
    }

    fn run(mut self) -> Result<(), Error> {
        let mut out = std::io::stdout();
        
        // 1. Initial State Setup
        enable_raw_mode().map_err(|e| Error::Platform(format!("Failed to enable raw mode: {}", e)))?;
        out.execute(EnterAlternateScreen).unwrap();
        out.execute(Hide).unwrap();
        
        register_redraw_proxy(Box::new(|| {}));

        // 2. Main Loop
        let res = (|| {
            loop {
                if rupa_motion::GLOBAL_TIMELINE.tick() { }
                self.handle_redraw();

                if event::poll(Duration::from_millis(32)).unwrap_or(false) {
                    match event::read().unwrap() {
                        Event::Key(key) => {
                            if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                                return Ok(());
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
        })();

        // 3. Guaranteed Cleanup Logic
        out.execute(Show).unwrap();
        out.execute(LeaveAlternateScreen).unwrap();
        let _ = disable_raw_mode();
        
        res
    }
}
