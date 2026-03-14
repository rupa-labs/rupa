use std::fs;
use std::path::Path;
use std::sync::Arc;
use rupa_base::Error;
use std::process::Command;

pub mod cargo;
pub mod entry;
pub mod component;

/// Stubs embedded into the binary at compile time.
pub mod stubs {
    pub const CARGO: &str = include_str!("../../stubs/cargo.stub");
    pub const CARGO_BASE: &str = include_str!("../../stubs/cargo_base.stub");
    pub const MAIN_PURE: &str = include_str!("../../stubs/main_pure.stub");
    pub const MAIN_NATIVE: &str = include_str!("../../stubs/main_native.stub");
    pub const MAIN_TERMINAL: &str = include_str!("../../stubs/main_terminal.stub");
    pub const MAIN_WEB: &str = include_str!("../../stubs/main_web.stub");
    pub const MAIN_SERVER: &str = include_str!("../../stubs/main_server.stub");
    pub const MAIN_MOBILE: &str = include_str!("../../stubs/main_mobile.stub");
    pub const MAIN_HYBRID: &str = include_str!("../../stubs/main_hybrid.stub");
    pub const MAIN_FULLSTACK: &str = include_str!("../../stubs/main_fullstack.stub");
    pub const MAIN_PLUGIN: &str = include_str!("../../stubs/main_plugin.stub");
    pub const APP_COMPONENT: &str = include_str!("../../stubs/app_component.stub");
}

/// Defines the project templates available for scaffolding.
#[derive(Debug, Clone)]
pub enum TemplateType {
    Pure,           // Zero Bloat
    Handcraft,      // Assembly Creation
    Native,         // Desktop
    Terminal,       // TUI
    Web,            // Web Excellence
    Mobile,         // Mobile Mobility
    Headless,       // Pure Logic
    Server,         // Server Authority
    Hybrid,         // Hybrid Interop
    Fullstack,      // Fullstack Fusion
    Plugin,         // Plugin Creation
    Library,        // Generic Rupa Library
    Git(String),    // External Template
}

pub struct Scaffolder;

impl Scaffolder {
    pub fn craft(
        project_name: &str, 
        template: TemplateType, 
        progress: Option<Arc<dyn Fn(f32, &str) + Send + Sync>>
    ) -> Result<(), Error> {
        let root = Path::new(project_name);
        let report = |val: f32, msg: &str| { if let Some(ref p) = progress { p(val, msg); } };

        if root.exists() {
            return Err(Error::Platform(format!("Directory '{}' already exists.", project_name)));
        }

        if let TemplateType::Git(url) = template {
            return Self::craft_from_git(project_name, &url, progress);
        }

        report(0.1, "Initializing workshop structure...");
        Self::create_dirs(root, &template)?;
        
        report(0.3, "Forging manifest (Cargo.toml)...");
        let cargo_toml = cargo::generate(project_name, &template);
        fs::write(root.join("Cargo.toml"), cargo_toml).map_err(|e| Error::Platform(e.to_string()))?;

        report(0.5, "Carving entry points (src/main.rs)...");
        let main_rs = entry::generate(project_name, &template);
        fs::write(root.join("src/main.rs"), main_rs).map_err(|e| Error::Platform(e.to_string()))?;

        if Self::needs_app_component(&template) {
            report(0.7, "Assembling base components...");
            fs::write(root.join("src/components/mod.rs"), "pub mod app;\npub use app::AppRoot;\n").unwrap();
            fs::write(root.join("src/components/app.rs"), component::generate()).unwrap();
        }

        report(0.9, "Finishing atmosphere (.gitignore)...");
        fs::write(root.join(".gitignore"), "/target\nCargo.lock\n.rupa_storage\n").unwrap();

        report(1.0, "Crafting complete!");
        Ok(())
    }

    fn create_dirs(root: &Path, template: &TemplateType) -> Result<(), Error> {
        let mut dirs = vec!["src", "assets", "docs"];
        if Self::needs_app_component(template) {
            dirs.push("src/components");
        }
        for dir in dirs {
            fs::create_dir_all(root.join(dir)).map_err(|e| Error::Platform(e.to_string()))?;
        }
        Ok(())
    }

    fn needs_app_component(template: &TemplateType) -> bool {
        match template {
            TemplateType::Pure | TemplateType::Headless | TemplateType::Plugin | TemplateType::Library => false,
            _ => true,
        }
    }

    fn craft_from_git(name: &str, url: &str, progress: Option<Arc<dyn Fn(f32, &str) + Send + Sync>>) -> Result<(), Error> {
        let report = |val: f32, msg: &str| { if let Some(ref p) = progress { p(val, msg); } };
        report(0.2, &format!("Pulling template from {}...", url));
        let status = Command::new("git").args(["clone", "--depth", "1", url, name]).status().map_err(|e| Error::Platform(e.to_string()))?;
        if !status.success() { return Err(Error::Platform("Git clone failed.".into())); }
        let dot_git = Path::new(name).join(".git");
        if dot_git.exists() { let _ = fs::remove_dir_all(dot_git); }
        report(1.0, "Crafting complete from Git!");
        Ok(())
    }
}
