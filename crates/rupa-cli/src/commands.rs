use rupa_core::{Component, VNode, ViewCore, Id, Signal};
use rupa_ui::elements::{VStack, Text, Button};
use rupa_engine::App;
use rupa_terminal::{TerminalRunner, Console};
use rupa_console::Progress;
use rupa_engine::platform::runner::PlatformRunner;
use rupa_signals::batch;
use std::sync::Arc;
use clap::{Parser, Subcommand};
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
        /// Template to use (skips wizard if name is also provided)
        #[arg(short, long)]
        template: Option<String>,
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
    progress: Progress,
}

impl CreateWizard {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            stage: Signal::new(WizardStage::Welcome),
            project_name: Signal::new("my-rupa-app".to_string()),
            selected_template: Signal::new(None),
            error_msg: Signal::new("".to_string()),
            view: Arc::new(ViewCore::new()),
            progress: Progress::new("Crafting"),
        }
    }
}

impl Component for CreateWizard {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }

    fn render(&self) -> VNode {
        let current_stage = self.stage.get();
        
        let root = match current_stage {
            WizardStage::Welcome => {
                let stage = self.stage.clone();
                VStack::new()
                    .child(Text::new("🎨 RUPA FRAMEWORK"))
                    .child(Text::new("The Artisan's Choice for Multi-platform Excellence."))
                    .child(Button::new("Begin Crafting →").on_click(move |_| stage.set(WizardStage::NameInput)))
            }
            WizardStage::NameInput => {
                let stage = self.stage.clone();
                let name = self.project_name.get();
                VStack::new()
                    .child(Text::new("PROJECT SIGNATURE"))
                    .child(Text::new(format!("Name: '{}'", name)))
                    .child(Button::new("Confirm Signature →").on_click(move |_| stage.set(WizardStage::TemplateSelection)))
            }
            WizardStage::TemplateSelection => {
                let stage = self.stage.clone();
                let selected = self.selected_template.clone();
                
                let mut list = VStack::new().child(Text::new("CHOOSE YOUR PALETTE"));
                
                let templates = vec![
                    "Showroom (Zero Bloat - Default)",
                    "Native Power (Desktop)",
                    "Web Excellence (Web/SSR)",
                    "Terminal Arts (TUI)",
                    "Composite (UI Library)"
                ];

                for (idx, name) in templates.into_iter().enumerate() {
                    let stage = stage.clone();
                    let selected = selected.clone();
                    list = list.child(Button::new(name).on_click(move |_| {
                        batch(|| {
                            selected.set(Some(idx));
                            stage.set(WizardStage::Scaffolding);
                        });
                    }));
                }
                
                list
            }
            WizardStage::Scaffolding => {
                // Background work trigger
                let stage = self.stage.clone();
                let error_msg = self.error_msg.clone();
                let project_name = self.project_name.get();
                let template_idx = self.selected_template.get().unwrap_or(0);
                
                // Clone signals for the thread
                let p_val = self.progress.value.clone();
                let p_label = self.progress.label.clone();
                
                std::thread::spawn(move || {
                    let template = match template_idx {
                        0 => TemplateType::ZeroBloat,
                        1 => TemplateType::Desktop,
                        2 => TemplateType::Web,
                        3 => TemplateType::Tui,
                        _ => TemplateType::Library,
                    };

                    let p_callback = Arc::new(move |val, msg: &str| {
                        p_val.set(val);
                        p_label.set(msg.to_string());
                    });

                    match Scaffolder::craft(&project_name, template, Some(p_callback)) {
                        Ok(_) => {
                            // Give user a moment to see 100%
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            stage.set(WizardStage::Finished);
                        },
                        Err(e) => {
                            error_msg.set(e.to_string());
                            stage.set(WizardStage::Error);
                        }
                    }
                });

                VStack::new()
                    .child(Text::new("ARTISAN AT WORK"))
                    .child(self.progress.clone())
            }
            WizardStage::Finished => {
                VStack::new()
                    .child(Text::new("PROJECT READY!"))
                    .child(Text::new(format!("Run: cd {} && cargo run", self.project_name.get())))
            }
            WizardStage::Error => {
                VStack::new()
                    .child(Text::new("CRAFTING FAILED"))
                    .child(Text::new(format!("Error: {}", self.error_msg.get())))
            }
        };

        root.render()
    }
}

pub async fn handle() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Create { name, template }) => {
            // Non-interactive flow
            if let (Some(project_name), Some(template_str)) = (&name, &template) {
                let template_type = match template_str.to_lowercase().as_str() {
                    "desktop" | "showroom" => TemplateType::Desktop,
                    "web" => TemplateType::Web,
                    "tui" | "terminal" => TemplateType::Tui,
                    "library" => TemplateType::Library,
                    _ => {
                        Console::error(format!("Unknown template type: '{}'. Valid types: desktop, web, terminal, library", template_str));
                        return Ok(());
                    }
                };

                Console::info(format!("Crafting project '{}' using {} template...", project_name, template_str));
                
                let p_callback = Arc::new(move |val: f32, msg: &str| {
                    println!("  [{:>3.0}%] {}", val * 100.0, msg);
                });

                match Scaffolder::craft(project_name, template_type, Some(p_callback)) {
                    Ok(_) => {
                        Console::success("Project successfully crafted!");
                        Console::text(format!("Run: cd {} && cargo run", project_name));
                    },
                    Err(e) => Console::error(format!("Crafting failed: {}", e)),
                }
                return Ok(());
            }

            // Interactive Wizard flow
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
                    let vnode = rupa_md::Engine::parse(&content, None);
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
