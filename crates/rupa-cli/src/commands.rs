use rupa_core::{Component, VNode, ViewCore, generate_id, Signal, vnode::Style, renderer::{Renderer, TextMeasurer}, scene::SceneNode};
use rupa_ui::elements::{VStack, Text, Button};
use rupa_engine::App;
use std::sync::Arc;
use clap::{Parser, Subcommand};
use crate::ui::ListSelector;
use crate::templates::{Scaffolder, TemplateType};

#[derive(Parser)]
#[command(name = "rupa")]
#[command(about = "Rupa Framework CLI - Craft with excellence.", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Rupa project with an aesthetic wizard.
    Create {
        /// Name of the project
        name: Option<String>,
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
}

#[derive(Clone, PartialEq, Default)]
enum WizardStage {
    #[default]
    Welcome,
    NameInput,
    TemplateSelection,
    Scaffolding,
    Finished,
    Error(String),
}

struct CreateWizard {
    id: String,
    stage: Signal<WizardStage>,
    project_name: Signal<String>,
    view: Arc<ViewCore>,
}

impl CreateWizard {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            stage: Signal::new(WizardStage::Welcome),
            project_name: Signal::new("my-rupa-app".into()),
            view: Arc::new(ViewCore::new()),
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
                    .child(Box::new(Text::new("PROJECT SIGNATURE")))
                    .child(Box::new(Text::new("Name: 'my-rupa-app'")))
                    .child(Box::new(Button::new("Confirm Signature →").on_click({
                        let stage = self.stage.clone();
                        move |_| stage.set(WizardStage::TemplateSelection)
                    })))
                    .render()
            }
            WizardStage::TemplateSelection => {
                VStack::new()
                    .child(Box::new(Text::new("CHOOSE YOUR TEMPLATE")))
                    .child(Box::new(ListSelector::new(vec![
                        "Showroom (Fullstack App)",
                        "Composite (UI Library)",
                        "Atomic (Custom Engine)"
                    ]).on_submit({
                        let stage = self.stage.clone();
                        move |_idx| {
                            stage.set(WizardStage::Scaffolding);
                        }
                    })))
                    .render()
            }
            WizardStage::Scaffolding => {
                VStack::new().child(Box::new(Text::new("CRAFTING..."))).render()
            }
            WizardStage::Finished => {
                VStack::new().child(Box::new(Text::new("PROJECT READY!"))).render()
            }
            WizardStage::Error(msg) => {
                VStack::new().child(Box::new(Text::new(format!("ERROR: {}", msg)))).render()
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

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &taffy::prelude::TaffyTree<()>, _node: taffy::prelude::NodeId, _is_group_hovered: bool, _global_pos: rupa_support::Vec2) {}
}

pub async fn handle() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name: _ } => {
            let wizard = CreateWizard::new();
            App::new("create-rupa-app")
                .root(wizard)
                .run_terminal();
        }
        Commands::Update { canary, to } => {
            println!("🔄 Refining your artisan tools...");
            
            let mut cmd = std::process::Command::new("cargo");
            cmd.arg("install");

            if canary {
                println!("🚀 Pulling the bleeding edge from the workshop (Git)...");
                cmd.args(["--git", "https://github.com/rupa-labs/rupa", "rupa-cli"]);
            } else if let Some(version) = to {
                println!("📍 Switching to version: {}...", version);
                cmd.args(["rupa-cli", "--version", &version]);
            } else {
                println!("📦 Fetching the latest stable release...");
                cmd.arg("rupa-cli");
            }

            let status = cmd.status();

            match status {
                Ok(s) if s.success() => println!("✨ Rupa CLI has been successfully refined."),
                _ => eprintln!("❌ Refinement failed. Please ensure Cargo is installed and you have network access."),
            }
        }
    }

    Ok(())
}
