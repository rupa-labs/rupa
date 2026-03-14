use rupa_core::{Component, VNode, Signal};
use rupa_ui::elements::{VStack, Text, Button, Input};
use rupa_engine::App;
use rupa_terminal::TerminalRunner;
use rupa_console::{Console, Progress};
use rupa_engine::platform::runner::PlatformRunner;
use rupa_signals::batch;
use std::sync::{Arc, RwLock};
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
    /// Clear terminal session, terminal cache, and project application cache.
    Clear {
        /// Also perform a deep clean (removes target directory).
        #[arg(long)]
        deep: bool,
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
    stage: Signal<WizardStage>,
    project_name: Signal<String>,
    selected_template: Signal<Option<usize>>,
    error_msg: Signal<String>,
    prev_vnode: Arc<RwLock<Option<VNode>>>,
    progress: Arc<Progress>, // Progress bar is now a component
}

impl CreateWizard {
    pub fn new() -> Self {
        Self {
            stage: Signal::new(WizardStage::Welcome),
            project_name: Signal::new("my-rupa-app".to_string()),
            selected_template: Signal::new(None),
            error_msg: Signal::new("".to_string()),
            prev_vnode: Arc::new(RwLock::new(None)),
            progress: Arc::new(Progress::new(Signal::new(0.0))),
        }
    }
}

impl Component for CreateWizard {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }

    fn render(&self) -> VNode {
        let current_stage = self.stage.get();
        
        let root: Box<dyn Component> = match current_stage {
            WizardStage::Welcome => {
                let stage = self.stage.clone();
                Box::new(VStack::new()
                    .child(Text::new("🎨 RUPA FRAMEWORK").bold())
                    .child(Text::new("The Artisan's Choice for Multi-platform Excellence."))
                    .child(Button::new("Begin Crafting →")
                        .with_key("btn-welcome")
                        .on_click(move |_| stage.set(WizardStage::NameInput))))
            }
            WizardStage::NameInput => {
                let stage = self.stage.clone();
                let project_name = self.project_name.clone();
                
                Box::new(VStack::new()
                    .child(Text::new("PROJECT SIGNATURE").bold())
                    .child(Text::new("Craft a unique identifier for your project.").dim())
                    .child(Input::new("Enter project name...")
                        .with_key("project-name-input")
                        .value(project_name)
                        .on_submit({
                            let stage = stage.clone();
                            move |_| stage.set(WizardStage::TemplateSelection)
                        })
                    )
                    .child(Button::new("Confirm Signature →")
                        .with_key("btn-name-confirm")
                        .on_click(move |_| stage.set(WizardStage::TemplateSelection))
                    ))
            }
            WizardStage::TemplateSelection => {
                let stage = self.stage.clone();
                let selected = self.selected_template.clone();
                
                let mut list = VStack::new().child(Text::new("CHOOSE YOUR PALETTE").bold());
                
                let templates = vec![
                    "Pure (Zero Bloat - Default)",
                    "Native Power (Desktop)",
                    "Web Excellence (Web/SSR)",
                    "Terminal Arts (TUI)",
                    "The Handcraft Path (Assembly)",
                    "Mobile Mobility (Mobile)",
                    "Pure Logic (Headless)",
                    "Server Authority (Server)",
                    "Hybrid Interop (Hybrid)",
                    "Fullstack Fusion (Fullstack)",
                    "Plugin Creation (Plugin)"
                ];

                for (idx, name) in templates.into_iter().enumerate() {
                    let stage = stage.clone();
                    let selected = selected.clone();
                    list = list.child(Button::new(name)
                        .with_key(format!("template-{}", idx))
                        .on_click(move |_| {
                            batch(|| {
                                selected.set(Some(idx));
                                stage.set(WizardStage::Scaffolding);
                            });
                        }));
                }
                
                Box::new(list)
            }
            WizardStage::Scaffolding => {
                let stage = self.stage.clone();
                let error_msg = self.error_msg.clone();
                let project_name = self.project_name.get();
                let template_idx = self.selected_template.get().unwrap_or(0);
                
                let p_val = self.progress.value.clone();
                
                std::thread::spawn(move || {
                    let template = match template_idx {
                        0 => TemplateType::Pure,
                        1 => TemplateType::Native,
                        2 => TemplateType::Web,
                        3 => TemplateType::Terminal,
                        4 => TemplateType::Handcraft,
                        5 => TemplateType::Mobile,
                        6 => TemplateType::Headless,
                        7 => TemplateType::Server,
                        8 => TemplateType::Hybrid,
                        9 => TemplateType::Fullstack,
                        10 => TemplateType::Plugin,
                        _ => TemplateType::Pure,
                    };

                    let p_callback = Arc::new(move |val, _msg: &str| {
                        p_val.set(val);
                    });

                    match Scaffolder::craft(&project_name, template, Some(p_callback)) {
                        Ok(_) => {
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            stage.set(WizardStage::Finished);
                        },
                        Err(e) => {
                            error_msg.set(e.to_string());
                            stage.set(WizardStage::Error);
                        }
                    }
                });

                Box::new(VStack::new()
                    .child(Text::new("ARTISAN AT WORK").bold())
                    // For now, we'll just show text as progress bar component is still evolving
                    .child(Text::new("Scaffolding project...")))
            }
            WizardStage::Finished => {
                Box::new(VStack::new()
                    .child(Text::new("PROJECT READY!").success())
                    .child(Text::new(format!("Run: cd {} && cargo run", self.project_name.get())))
                    .child(Button::new("Exit Wizard")
                        .with_key("btn-exit-success")
                        .on_click(|_| std::process::exit(0))))
            }
            WizardStage::Error => {
                Box::new(VStack::new()
                    .child(Text::new("CRAFTING FAILED").error())
                    .child(Text::new(format!("Error: {}", self.error_msg.get())))
                    .child(Button::new("Exit")
                        .with_key("btn-exit-error")
                        .on_click(|_| std::process::exit(1))))
            }
        };

        root.render()
    }
}

pub async fn handle() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Create { name, template }) => {
            if let (Some(project_name), Some(template_str)) = (&name, &template) {
                let template_type = if template_str.starts_with("http") || template_str.starts_with("git@") {
                    TemplateType::Git(template_str.clone())
                } else {
                    match template_str.to_lowercase().as_str() {
                        "pure" => TemplateType::Pure,
                        "handcraft" => TemplateType::Handcraft,
                        "native" | "desktop" => TemplateType::Native,
                        "terminal" | "tui" => TemplateType::Terminal,
                        "web" => TemplateType::Web,
                        "mobile" => TemplateType::Mobile,
                        "headless" => TemplateType::Headless,
                        "server" => TemplateType::Server,
                        "hybrid" => TemplateType::Hybrid,
                        "fullstack" => TemplateType::Fullstack,
                        "plugin" => TemplateType::Plugin,
                        "library" => TemplateType::Library,
                        _ => {
                            Console::error(format!("Unknown template type: '{}'.", template_str));
                            return Ok(());
                        }
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

            Console::info("Initializing Artisan Wizard...");
            let wizard = CreateWizard::new();

            if let Some(project_name) = name {
                wizard.project_name.set(project_name);
                wizard.stage.set(WizardStage::TemplateSelection);
            }

            let app = App::new("create-rupa-app")
                .root(std::sync::Arc::new(wizard));

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
                    perform_post_update_cleanup();
                }
                _ => {
                    if !canary && to.is_none() {
                        Console::info("Stable release not found in registry. Redirecting to artisan repository...");
                        let git_status = std::process::Command::new("cargo")
                            .args(["install", "--git", "https://github.com/rupa-labs/rupa", "rupa-cli"])
                            .status();

                        match git_status {
                            Ok(s) if s.success() => {
                                Console::success("Rupa CLI has been successfully refined from repository.");
                                perform_post_update_cleanup();
                            },
                            _ => Console::error("Refinement failed. Please ensure Cargo is installed and you have network access."),
                        }
                    } else {
                        Console::error("Refinement failed. Please ensure Cargo is installed and you have network access.");
                    }
                }
            }
        }
        Some(Commands::Clear { deep }) => {
            Console::info("Purifying artisan environment...");
            
            print!("\x1B[2J\x1B[H"); 

            let local_targets = vec![".rupa_storage", "dist"];
            for target in local_targets {
                if std::path::Path::new(target).exists() {
                    if let Err(e) = std::fs::remove_dir_all(target) {
                        Console::error(format!("  - Failed to purge {}: {}", target, e));
                    } else {
                        Console::text(format!("  - Purged local artifact: {}", target));
                    }
                }
            }

            if deep {
                if std::path::Path::new("target").exists() {
                    Console::info("Removing heavy build artifacts (target/)...");
                    if let Err(e) = std::fs::remove_dir_all("target") {
                        Console::error(format!("  - Failed to purge target/: {}", e));
                    } else {
                        Console::text("  - Purged build artifacts: target/");
                    }
                }
            }

            let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE"));
            if let Ok(home_path) = home {
                let rupa_cache = std::path::Path::new(&home_path).join(".rupa").join("cache");
                if rupa_cache.exists() {
                    if let Err(e) = std::fs::remove_dir_all(&rupa_cache) {
                        Console::error(format!("  - Failed to purge global cache: {}", e));
                    } else {
                        Console::text("  - Purged global session cache.");
                    }
                }
            }

            Console::success("Environment purified successfully.");
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

fn perform_post_update_cleanup() {
    print!("\x1B[2J\x1B[H"); 
    
    Console::draw_box("REFINEMENT COMPLETE", vec![
        "Internal session caches have been cleared.".to_string(),
        "To apply changes immediately in this shell, run:".to_string(),
        "".to_string(),
        "  hash -r (bash)  OR  rehash (zsh)".to_string(),
    ]);
}
