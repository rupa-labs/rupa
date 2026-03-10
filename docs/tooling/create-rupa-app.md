# create-rupa-app 🎨

`create-rupa-app` is the official, aesthetic initialization wizard for the Rupa Framework. It transforms the often mundane task of project setup into a delightful "crafting" experience directly in your terminal.

---

## 🏗️ The Crafting Experience

The wizard is powered by the **Rupa TUI** engine, ensuring that your first interaction with the framework is as high-quality as the applications you will build with it.

### Key Features:
- **Interactive TUI**: Navigate through options using your keyboard (`Arrows` to move, `Enter` to select).
- **Reactive Workflow**: The interface responds instantly to your choices.
- **Aesthetic Design**: Carefully crafted terminal layouts with semantic coloring.
- **Tiered Scaffolding**: Specialized templates based on the **Artisan Workshop Standard**.

---

## 🚀 Getting Started

### 1. Installation
To install the Rupa CLI and the `create` tool, ensure you have Rust and Cargo installed, then run:

```bash
cargo install rupa-cli
```

### 2. Launching the Wizard
Start the crafting process by running:

```bash
rupa create
```

---

## 🎨 Choose Your Palette (Templates)

During the setup, you will be asked to choose a starting template. Each corresponds to a tier in the Rupa ecosystem:

| Template | Best For... | Included Materials |
| :--- | :--- | :--- |
| **Showroom (Fullstack)** | Building complete, cross-platform apps. | Core, UI, Motion, Auth, Net. |
| **Composite (Library)** | Creating a reusable suite of UI components. | Core, UI, Signals. |
| **Atomic (Engine)** | Building a custom rendering engine or low-level tool. | Core, Signals, Support. |
| **Artisan (Minimalist)** | A blank canvas for experimental creations. | Core only. |

---

## 📁 Project Anatomy

Once the wizard completes its work, your new project will have the following structure:

```text
my-rupa-app/
├── src/
│   └── main.rs       # The heart of your application
├── Cargo.toml        # Manifest with Rupa dependencies
└── .gitignore        # Standard Rust ignores
```

### Next Steps:
```bash
cd my-rupa-app
cargo run
```

---

## 💡 Artisan Tips

- **Fullscreen Bliss**: For the best experience, run the wizard in a modern terminal (like Alacritty, Kitty, or iTerm2) with a decent width.
- **Navigation**: Use the `Up` and `Down` arrows to browse templates. The selection is highlighted in **Artisan Blue**.
- **Exit**: You can gracefully cancel the process at any time by pressing `Ctrl + C`.

---

*Crafted with excellence by the Rupa Community.*
