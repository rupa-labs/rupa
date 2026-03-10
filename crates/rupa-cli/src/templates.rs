use std::fs;
use std::path::Path;
use rupa_support::Error;

/// Defines the project templates available for scaffolding.
pub enum TemplateType {
    Showroom,
    Composite,
    Atomic,
}

pub struct Scaffolder;

impl Scaffolder {
    /// Creates a new project structure on the disk.
    pub fn craft(project_name: &str, template: TemplateType) -> Result<(), Error> {
        let root = Path::new(project_name);
        
        if root.exists() {
            return Err(Error::Platform(format!("Directory '{}' already exists.", project_name)));
        }

        // 1. Create directory structure
        fs::create_dir_all(root.join("src")).map_err(|e| Error::Platform(e.to_string()))?;
        
        // 2. Generate Cargo.toml
        let cargo_toml = match template {
            TemplateType::Showroom => Self::showroom_cargo(project_name),
            _ => Self::base_cargo(project_name),
        };
        fs::write(root.join("Cargo.toml"), cargo_toml).map_err(|e| Error::Platform(e.to_string()))?;

        // 3. Generate main.rs
        let main_rs = match template {
            TemplateType::Showroom => Self::showroom_main(),
            _ => Self::base_main(),
        };
        fs::write(root.join("src/main.rs"), main_rs).map_err(|e| Error::Platform(e.to_string()))?;

        // 4. Generate .gitignore
        fs::write(root.join(".gitignore"), "/target\nCargo.lock").unwrap();

        Ok(())
    }

    fn showroom_cargo(name: &str) -> String {
        format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
rupa = {{ git = "https://github.com/rupa-labs/rupa" }}
"# , name)
    }

    fn base_cargo(name: &str) -> String {
        format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
rupa-core = {{ git = "https://github.com/rupa-labs/rupa" }}
"# , name)
    }

    fn showroom_main() -> String {
        r#"use rupa::prelude::*;

fn main() {
    let mut app = App::new(Box::new(MyComponent::new()));
    app.run().unwrap();
}

struct MyComponent { id: String }
impl MyComponent { fn new() -> Self { Self { id: generate_id() } } }
impl Component for MyComponent {
    fn id(&self) -> &str { &self.id }
    fn render(&self) -> VNode {
        VNode::text("Welcome to Rupa!")
    }
}
"#.to_string()
    }

    fn base_main() -> String {
        "fn main() { println!(\"Rupa project initialized.\"); }".to_string()
    }
}
