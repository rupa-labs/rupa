# Composition: Universal Routing 🚦

The Rupa Framework Router is a declarative navigation system designed to work seamlessly across Desktop, Terminal, Web, and Mobile. It utilizes the **Signal Engine** to provide fine-grained, reactive view switching.

---

## 🏗️ Architecture

The router follows a **Logic-First** approach. It is a specialized component that listens to a global `path` signal and renders the matching child tree.

### Key Components
- **`Router`**: The orchestrator that holds the navigation state and manages platform synchronization.
- **`Route`**: A declarative definition of a path and its associated component builder.
- **`Link`**: An interactive element that triggers path changes without page refreshes.

---

## 🌐 Platform Synchronization

The Router's behavior adapts based on the target platform:

| Platform | Navigation Mechanism | URL Sync |
| :--- | :--- | :--- |
| **Web** | Browser History API (`popstate`) | Yes (`window.location`) |
| **Desktop** | Internal State Signal | No |
| **Terminal** | Internal State Signal | No |
| **Mobile** | Stack-based State Signal | Limited |

---

## 🗝️ API & Usage

### Declarative Routing
Define your application structure inside the `Body`:

```rust
Router::new()
    .route("/", |_| Box::new(HomeView::new()))
    .route("/settings", |_| Box::new(SettingsView::new()))
    .fallback(|_| Box::new(NotFoundView::new()))
```

### Programmatic Navigation
You can trigger navigation from any event handler via the shared context:

```rust
button.on_click(|_| {
    Router::push("/settings");
})
```

---

## 🛡️ Web Integration (WASM)

On the **Web target**, the Router automatically hooks into the `window.onpopstate` event. When the user clicks the browser's "Back" button, the internal `path` signal is updated, triggering a reactive re-render of the appropriate `Route`. 

Directly modifying the URL via the address bar also synchronizes the initial state of the application during bootstrap.
