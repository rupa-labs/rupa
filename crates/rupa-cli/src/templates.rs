use std::fs;
use std::path::Path;
use rupa_base::Error;

/// Defines the project templates available for scaffolding.
#[derive(Debug, Clone, Copy)]
pub enum TemplateType {
    ZeroBloat,
    Desktop,
    Web,
    Tui,
    Library,
}

pub struct Scaffolder;

impl Scaffolder {
    /// Creates a new project structure on the disk.
    pub fn craft(project_name: &str, template: TemplateType) -> Result<(), Error> {
        let root = Path::new(project_name);
        
        if root.exists() {
            return Err(Error::Platform(format!("Directory '{}' already exists.", project_name)));
        }

        // 1. Create base directory structure
        Self::create_dirs(root)?;
        
        // 2. Generate Cargo.toml
        let cargo_toml = match template {
            TemplateType::ZeroBloat => Self::showroom_cargo(project_name, &["desktop"]),
            TemplateType::Desktop => Self::showroom_cargo(project_name, &["desktop"]),
            TemplateType::Web => Self::showroom_cargo(project_name, &["web", "ssr"]),
            TemplateType::Tui => Self::showroom_cargo(project_name, &["terminal"]),
            TemplateType::Library => Self::base_cargo(project_name),
        };
        fs::write(root.join("Cargo.toml"), cargo_toml).map_err(|e| Error::Platform(e.to_string()))?;

        // 3. Generate src/main.rs
        let main_rs = Self::generate_main(&template);
        fs::write(root.join("src/main.rs"), main_rs).map_err(|e| Error::Platform(e.to_string()))?;

        // 4. Generate src/components/mod.rs and src/components/app.rs
        if !matches!(template, TemplateType::Library) {
            fs::write(root.join("src/components/mod.rs"), "pub mod app;\npub use app::AppRoot;\n").unwrap();
            fs::write(root.join("src/components/app.rs"), Self::generate_app_component()).unwrap();
        }

        // 5. Generate .gitignore
        fs::write(root.join(".gitignore"), "/target\nCargo.lock\n.rupa_storage\n").unwrap();

        // 6. Generate GEMINI.md (Standard Compliance)
        fs::write(root.join("GEMINI.md"), Self::generate_gemini_md(project_name)).unwrap();

        Ok(())
    }

    fn create_dirs(root: &Path) -> Result<(), Error> {
        let dirs = ["src", "src/components", "assets", "docs"];
        for dir in dirs {
            fs::create_dir_all(root.join(dir)).map_err(|e| Error::Platform(e.to_string()))?;
        }
        Ok(())
    }

    fn showroom_cargo(name: &str, features: &[&str]) -> String {
        let features_str = features.iter()
            .map(|f| format!("\"{}\"", f))
            .collect::<Vec<_>>()
            .join(", ");

        format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
authors = ["Artisan"]
description = "A new Rupa project."

[dependencies]
rupa = {{ git = "https://github.com/rupa-labs/rupa", features = [{}] }}
serde = {{ version = "1.0", features = ["derive"] }}
log = "0.4"
env_logger = "0.10"
"# , name, features_str)
    }

    fn base_cargo(name: &str) -> String {
        format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
rupa-base = {{ git = "https://github.com/rupa-labs/rupa" }}
rupa-signals = {{ git = "https://github.com/rupa-labs/rupa" }}
"# , name)
    }

    fn generate_main(template: &TemplateType) -> String {
        match template {
            TemplateType::Library => {
                "pub fn init() { println!(\"Rupa Library Initialized\"); }".to_string()
            }
            TemplateType::Tui => {
                r#"use rupa::prelude::*;
mod components;

fn main() {
    env_logger::init();
    
    let app = App::new("Rupa Terminal")
        .root(components::AppRoot::new());

    rupa::terminal::TerminalRunner::new(app.core.clone())
        .run()
        .expect("Failed to run terminal showroom");
}
"#.to_string()
            }
            _ => {
                r#"use rupa::prelude::*;
mod components;

fn main() {
    env_logger::init();

    App::new("Artisan Desktop")
        .root(components::AppRoot::new())
        .run()
        .expect("Failed to run desktop showroom");
}
"#.to_string()
            }
        }
    }

    fn generate_app_component() -> String {
        r#"use rupa::prelude::*;

pub struct AppRoot {
    id: String,
    view: std::sync::Arc<ViewCore>,
    count: Signal<i32>,
}

impl AppRoot {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            view: std::sync::Arc::new(ViewCore::new()),
            count: Signal::new(0),
        }
    }
}

impl Component for AppRoot {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> std::sync::Arc<ViewCore> { self.view.clone() }

    fn render(&self) -> VNode {
        let count = self.count.clone();
        
        VStack::new()
            .p(32.0)
            .items_center()
            .gap(16.0)
            .child(Text::new("🎨 RUPA FRAMEWORK")
                .style(font_bold())
                .style(text_xl2())
            )
            .child(Text::new(Memo::new(move || format!("Count: {}", count.get()))))
            .child(HStack::new()
                .gap(8.0)
                .child(Button::new("Decrement").on_click({
                    let c = self.count.clone();
                    move |_| c.update(|v| *v -= 1)
                }))
                .child(Button::new("Increment").on_click({
                    let c = self.count.clone();
                    move |_| c.update(|v| *v += 1)
                }))
            )
            .render()
    }
}
"#.to_string()
    }

    fn generate_gemini_md(name: &str) -> String {
        format!(r#"# {} - Project Context

This project is built using the **Rupa Framework** and follows the **Atoms and Composites** architectural model.

## Engineering Standards
- **Atoms (Tier 1)**: Pure, agnostic data and ports.
- **Composites (Tier 2)**: Business logic and assembly.
- **Showrooms (Tier 3)**: Platform-specific presentation.

## Maintenance Mandates
- Adhere to the **3S Doctrine**: Secure, Sustain, Scalable.
- Use **Fine-Grained Reactivity** (Signals) for all state management.
- Keep the `Component` logic pure and platform-agnostic.
"#, name)
    }
}
