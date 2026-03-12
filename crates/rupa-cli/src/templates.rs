use std::fs;
use std::path::Path;
use rupa_base::Error;

/// Defines the project templates available for scaffolding.
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

        // 1. Create directory structure
        fs::create_dir_all(root.join("src")).map_err(|e| Error::Platform(e.to_string()))?;
        
        // 2. Generate Cargo.toml
        let cargo_toml = match template {
            TemplateType::ZeroBloat => Self::showroom_cargo(project_name, &["desktop"]),
            TemplateType::Desktop => Self::showroom_cargo(project_name, &["desktop"]),
            TemplateType::Web => Self::showroom_cargo(project_name, &["web", "ssr"]),
            TemplateType::Tui => Self::showroom_cargo(project_name, &["tui"]),
            TemplateType::Library => Self::base_cargo(project_name),
        };
        fs::write(root.join("Cargo.toml"), cargo_toml).map_err(|e| Error::Platform(e.to_string()))?;

        // 3. Generate main.rs
        let main_rs = match template {
            TemplateType::Library => Self::base_main(),
            _ => Self::showroom_main(&template),
        };
        fs::write(root.join("src/main.rs"), main_rs).map_err(|e| Error::Platform(e.to_string()))?;

        // 4. Generate .gitignore
        fs::write(root.join(".gitignore"), "/target\nCargo.lock").unwrap();

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

[dependencies]
rupa = {{ git = "https://github.com/rupa-labs/rupa", features = [{}] }}
"# , name, features_str)
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

    fn showroom_main(template: &TemplateType) -> String {
        let run_method = match template {
            TemplateType::Tui => "run_terminal()",
            _ => "run()",
        };

        format!(r#"use rupa::prelude::*;

fn main() {{
    App::new("Artisan Project")
        .root(MyComponent::new())
        .{}
        .expect("Failed to run application");
}}

struct MyComponent {{ id: String }}
impl MyComponent {{ fn new() -> Self {{ Self {{ id: Id::next().to_string() }} }} }}
impl Component for MyComponent {{
    fn id(&self) -> &str {{ &self.id }}
    fn render(&self) -> VNode {{
        VNode::element("div")
            .with_style(Style::default().p(40.0).items_center())
            .with_child(VNode::text("Crafted with Rupa 🎨"))
    }}

    // Bridge methods for native rendering
    fn get_node(&self) -> Option<SceneNode> {{ None }}
    fn set_node(&self, _node: SceneNode) {{}}
    fn is_dirty(&self) -> bool {{ false }}
    fn mark_dirty(&self) {{}}
    fn clear_dirty(&self) {{}}
    fn layout(&self, taffy: &mut taffy::prelude::TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<taffy::prelude::NodeId>) -> taffy::prelude::NodeId {{
        taffy.new_leaf(taffy::prelude::Style::default()).unwrap()
    }}
    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &taffy::prelude::TaffyTree<()>, _node: taffy::prelude::NodeId, _is_group_hovered: bool, _global_pos: rupa_base::Vec2) {{}}
}}
"# , run_method)
    }

    fn base_main() -> String {
        "fn main() { println!(\"Rupa library initialized.\"); }".to_string()
    }
}
