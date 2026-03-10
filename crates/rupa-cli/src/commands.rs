use rupa_core::{Component, VNode, ViewCore, generate_id, Signal, renderer::{Renderer, TextMeasurer}, scene::SceneNode};
use rupa_ui::elements::{VStack, Text, Button};
use rupa_engine::App;
use rupa_terminal::{TerminalRunner, Console};
use rupa_engine::platform::runner::PlatformRunner;
use rupa_signals::Effect;
use std::sync::Arc;
use clap::{Parser, Subcommand};
use crate::ui::ListSelector;
use crate::templates::{Scaffolder, TemplateType};

#[derive(Parser)]
#[command(name = "rupa")]
#[command(version)]
#[command(about = "Rupa Framework CLI - Craft with excellence.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Rupa project with an aesthetic wizard.
    Create {
        /// Name of the project
        name: Option<String>,
    },
    /// Build the project for production (Static Site Generation).
    Build,
    /// Run a custom project action (Artisan Action).
    Run {
        /// The name of the action to run.
        action: String,
        /// The JSON payload for the action.
        #[arg(long)]
        payload: Option<String>,
    },
    /// Update the Rupa CLI to the latest version.
    Update {
        /// Update to the absolute latest version from the main Git repository.
        #[arg(long)]
        canary: bool,
        /// Switch to a specific version.
        #[arg(long)]
        to: Option<String>,
    },
    /// Display version information about Rupa CLI and Engine.
    Version,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
enum WizardStage {
    #[default]
    Welcome,
    NameInput,
    TemplateSelection,
    Scaffolding,
    Finished,
    Error,
}

struct CreateWizard {
    id: String,
    stage: Signal<WizardStage>,
    project_name: Signal<String>,
    selected_template: Signal<Option<usize>>,
    error_msg: Signal<String>,
    view: Arc<ViewCore>,
    _effect: Arc<Effect>,
}

impl CreateWizard {
    pub fn new() -> Self {
        let stage = Signal::new(WizardStage::Welcome);
        let project_name = Signal::new("my-rupa-app".to_string());
        let selected_template = Signal::new(None);
        let error_msg = Signal::new("".to_string());
        
        let wizard_state = (stage.clone(), project_name.clone(), selected_template.clone(), error_msg.clone());
        
        let effect = Effect::new(move || {
            let (stage, project_name, selected_template, error_msg) = wizard_state.clone();
            if stage.get() == WizardStage::Scaffolding {
                if let Some(idx) = selected_template.get() {
                    let name = project_name.get();
                    let template = match idx {
                        0 => TemplateType::ZeroBloat,
                        1 => TemplateType::Desktop,
                        2 => TemplateType::Web,
                        3 => TemplateType::Tui,
                        _ => TemplateType::Library,
                    };

                    match Scaffolder::craft(&name, template) {
                        Ok(_) => stage.set(WizardStage::Finished),
                        Err(e) => {
                            error_msg.set(e.to_string());
                            stage.set(WizardStage::Error);
                        }
                    }
                }
            }
        });

        Self {
            id: generate_id(),
            stage,
            project_name,
            selected_template,
            error_msg,
            view: Arc::new(ViewCore::new()),
            _effect: Arc::new(effect),
        }
    }
}

impl Component for CreateWizard {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }

    fn render(&self) -> VNode {
        let stage = self.stage.get();
        
        let content = match stage {
            WizardStage::Welcome => {
                VStack::new()
                    .child(Box::new(Text::new("🎨 RUPA FRAMEWORK")))
                    .child(Box::new(Text::new("The Artisan's Choice for Multi-platform Excellence.")))
                    .child(Box::new(Button::new("Begin Crafting →").on_click({
                        let stage = self.stage.clone();
                        move |_| stage.set(WizardStage::NameInput)
                    })))
                    .render()
            }
            WizardStage::NameInput => {
                VStack::new()
                    .gap(12.0)
                    .child(Box::new(Text::new("PROJECT SIGNATURE")))
                    .child(Box::new(Text::new(format!("Name: '{}'", self.project_name.get()))))
                    .child(Box::new(Button::new("Confirm Signature →").on_click({
                        let stage = self.stage.clone();
                        move |_| stage.set(WizardStage::TemplateSelection)
                    })))
                    .render()
            }
            WizardStage::TemplateSelection => {
                VStack::new()
                    .gap(12.0)
                    .child(Box::new(Text::new("CHOOSE YOUR PALETTE")))
                    .child(Box::new(ListSelector::new(vec![
                        "Showroom (Zero Bloat - Default)",
                        "Native Power (Desktop)",
                        "Web Excellence (Web/SSR)",
                        "Terminal Arts (TUI)",
                        "Composite (UI Library)"
                    ]).on_submit({
                        let stage = self.stage.clone();
                        let selected = self.selected_template.clone();
                        move |idx| {
                            selected.set(Some(idx));
                            stage.set(WizardStage::Scaffolding);
                        }
                    })))
                    .render()
            }
            WizardStage::Scaffolding => {
                VStack::new().child(Box::new(Text::new("CRAFTING..."))).render()
            }
            WizardStage::Finished => {
                VStack::new()
                    .child(Box::new(Text::new("PROJECT READY!")))
                    .child(Box::new(Text::new(format!("Run: cd {} && cargo run", self.project_name.get()))))
                    .render()
            }
            WizardStage::Error => {
                VStack::new()
                    .child(Box::new(Text::new("CRAFTING FAILED")))
                    .child(Box::new(Text::new(format!("Error: {}", self.error_msg.get()))))
                    .render()
            }
        };

        VNode::element("div")
            .with_style(self.view.style().clone())
            .with_child(content)
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }

    fn layout(&self, taffy: &mut taffy::prelude::TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<taffy::prelude::NodeId>) -> taffy::prelude::NodeId {
        let node = taffy.new_leaf(self.view.style().to_taffy()).unwrap();
        self.view.set_node(SceneNode::from(node));
        node
    }

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &taffy::prelude::TaffyTree<()>, _node: taffy::prelude::NodeId, _is_group_hovered: bool, _global_pos: rupa_base::Vec2) {}
}

impl Clone for CreateWizard {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            stage: self.stage.clone(),
            project_name: self.project_name.clone(),
            selected_template: self.selected_template.clone(),
            error_msg: self.error_msg.clone(),
            view: self.view.clone(),
            _effect: self._effect.clone(),
        }
    }
}

pub async fn handle() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Create { name }) => {
            Console::info("Initializing Artisan Wizard...");
            let wizard = CreateWizard::new();

            if let Some(project_name) = name {
                wizard.project_name.set(project_name);
                wizard.stage.set(WizardStage::TemplateSelection);
            }

            let app = App::new("create-rupa-app")
                .root(wizard);

            let runner = TerminalRunner::new(app.core.clone());
            if let Err(e) = runner.run() {
                Console::error(format!("Wizard initialization failed: {}", e));
            }
        }
        Some(Commands::Build) => {
            Console::info("Building your Artisan Site...");
            
            let pages_dir = std::path::Path::new("src/pages");
            let dist_dir = std::path::Path::new("dist");

            if !pages_dir.exists() {
                Console::error("'src/pages' directory not found. Is this a Rupa project?");
                return Ok(());
            }

            std::fs::create_dir_all(dist_dir).unwrap();

            let entries = std::fs::read_dir(pages_dir).unwrap();
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                
                if path.extension().map_or(false, |ext| ext == "md") {
                    let name = path.file_stem().unwrap().to_str().unwrap();
                    Console::text(format!("📄 Processing {}...", name));

                    let content = std::fs::read_to_string(&path).unwrap();
                    let vnode = rupa_md::MarkdownEngine::parse(&content);
                    let html = rupa_server::HtmlRenderer::render_vnode(&vnode);

                    let output_path = dist_dir.join(format!("{}.html", name));
                    std::fs::write(output_path, html).unwrap();
                }
            }

            Console::success("Build complete! Your site is ready in 'dist/'.");
        }
        Some(Commands::Run { action, payload }) => {
            Console::info(format!("Dispatching Artisan Action: {}...", action));
            
            let mut cmd = std::process::Command::new("cargo");
            cmd.arg("run");
            cmd.arg("--");
            cmd.arg("--rupa-action");
            cmd.arg(action);
            
            if let Some(data) = payload {
                cmd.arg("--rupa-payload");
                cmd.arg(data);
            }

            let status = cmd.status();
            if let Err(e) = status {
                Console::error(format!("Failed to execute action: {}", e));
            }
        }
        Some(Commands::Update { canary, to }) => {
            Console::info("Refining your artisan tools...");

            let mut cmd = std::process::Command::new("cargo");
            cmd.arg("install");

            if canary {
                Console::info("Pulling the bleeding edge from the workshop (Git)...");
                cmd.args(["--git", "https://github.com/rupa-labs/rupa", "rupa-cli"]);
            } else if let Some(ref version) = to {
                Console::info(format!("Switching to version: {}...", version));
                cmd.args(["rupa-cli", "--version", version]);
            } else {
                Console::info("Fetching the latest stable release from registry...");
                cmd.arg("rupa-cli");
            }

            let status = cmd.status();

            match status {
                Ok(s) if s.success() => {
                    Console::success("Rupa CLI has been successfully refined.");
                }
                _ => {
                    if !canary && to.is_none() {
                        Console::info("Stable release not found in registry. Redirecting to artisan repository...");
                        let git_status = std::process::Command::new("cargo")
                            .args(["install", "--git", "https://github.com/rupa-labs/rupa", "rupa-cli"])
                            .status();

                        match git_status {
                            Ok(s) if s.success() => Console::success("Rupa CLI has been successfully refined from repository."),
                            _ => Console::error("Refinement failed. Please ensure Cargo is installed and you have network access."),
                        }
                    } else {
                        Console::error("Refinement failed. Please ensure Cargo is installed and you have network access.");
                    }
                }
            }
        }
        Some(Commands::Version) | None => {
            Console::draw_box("RUPA FRAMEWORK", vec![
                format!("CLI Version:    {}", env!("CARGO_PKG_VERSION")),
                format!("Engine Version: {}", env!("CARGO_PKG_VERSION")),
                "Artisan Tier:   Showroom".to_string(),
                "Workshop:       https://github.com/rupa-labs/rupa".to_string(),
            ]);
            
            if cli.command.is_none() {
                Console::text("\nUsage: rupa <COMMAND>");
                Console::text("Run 'rupa --help' for more information.");
            }
        }
    }

    Ok(())
}
