use std::fs;
use std::path::Path;
use std::sync::Arc;
use rupa_base::Error;
use std::process::Command;

/// Defines the project templates available for scaffolding.
#[derive(Debug, Clone)]
pub enum TemplateType {
    ZeroBloat,
    Desktop,
    Web,
    Tui,
    Library,
    /// A custom template from a Git repository.
    Git(String),
}

pub struct Scaffolder;

impl Scaffolder {
    /// Creates a new project structure on the disk.
    pub fn craft(
        project_name: &str, 
        template: TemplateType, 
        progress: Option<Arc<dyn Fn(f32, &str) + Send + Sync>>
    ) -> Result<(), Error> {
        let root = Path::new(project_name);
        
        let report = |val: f32, msg: &str| {
            if let Some(ref p) = progress {
                p(val, msg);
            }
        };

        if root.exists() {
            return Err(Error::Platform(format!("Directory '{}' already exists.", project_name)));
        }

        // Handle Git Template separately
        if let TemplateType::Git(url) = template {
            return Self::craft_from_git(project_name, &url, progress);
        }

        report(0.1, "Initializing directory structure...");
        // 1. Create base directory structure
        Self::create_dirs(root)?;
        
        report(0.3, "Generating manifest (Cargo.toml)...");
        // 2. Generate Cargo.toml
        let cargo_toml = match template {
            TemplateType::ZeroBloat | TemplateType::Desktop => Self::showroom_cargo(project_name, &["desktop"]),
            TemplateType::Web => Self::showroom_cargo(project_name, &["web", "ssr"]),
            TemplateType::Tui => Self::showroom_cargo(project_name, &["terminal"]),
            TemplateType::Library => Self::base_cargo(project_name),
            TemplateType::Git(_) => unreachable!(), // Handled above
        };
        fs::write(root.join("Cargo.toml"), cargo_toml).map_err(|e| Error::Platform(e.to_string()))?;

        report(0.5, "Crafting artisan entry points (main.rs)...");
        // 3. Generate src/main.rs
        let main_rs = Self::generate_main(&template);
        fs::write(root.join("src/main.rs"), main_rs).map_err(|e| Error::Platform(e.to_string()))?;

        report(0.7, "Assembling components...");
        if !matches!(template, TemplateType::Library) {
            fs::write(root.join("src/components/mod.rs"), "pub mod app;\npub use app::AppRoot;\n").unwrap();
            fs::write(root.join("src/components/app.rs"), Self::generate_app_component()).unwrap();
        }

        report(0.9, "Finalizing environment (.gitignore, GEMINI.md)...");
        fs::write(root.join(".gitignore"), "/target\nCargo.lock\n.rupa_storage\n").unwrap();
        fs::write(root.join("GEMINI.md"), Self::generate_gemini_md(project_name)).unwrap();

        report(1.0, "Crafting complete!");
        Ok(())
    }

    fn craft_from_git(
        name: &str, 
        url: &str, 
        progress: Option<Arc<dyn Fn(f32, &str) + Send + Sync>>
    ) -> Result<(), Error> {
        let report = |val: f32, msg: &str| {
            if let Some(ref p) = progress {
                p(val, msg);
            }
        };

        report(0.2, &format!("Pulling template from {}...", url));
        
        let status = Command::new("git")
            .args(["clone", "--depth", "1", url, name])
            .status()
            .map_err(|e| Error::Platform(format!("Failed to execute git clone: {}", e)))?;

        if !status.success() {
            return Err(Error::Platform("Git clone failed. Please check the URL and your connection.".into()));
        }

        report(0.7, "Cleaning up git metadata...");
        let dot_git = Path::new(name).join(".git");
        if dot_git.exists() {
            let _ = fs::remove_dir_all(dot_git);
        }

        report(0.9, "Re-initializing project identity...");
        // Update Cargo.toml [package] name automatically
        let cargo_path = Path::new(name).join("Cargo.toml");
        if cargo_path.exists() {
            if let Ok(content) = fs::read_to_string(&cargo_path) {
                let mut new_content = String::new();
                let mut replaced = false;
                for line in content.lines() {
                    if !replaced && line.trim().starts_with("name =") {
                        new_content.push_str(&format!("name = \"{}\"\n", name));
                        replaced = true;
                    } else {
                        new_content.push_str(line);
                        new_content.push_str("\n");
                    }
                }
                let _ = fs::write(cargo_path, new_content);
            }
        }
        
        report(1.0, "Crafting complete from Git!");
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
