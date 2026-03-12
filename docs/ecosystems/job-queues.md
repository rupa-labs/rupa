# Job Queues & Background Tasks 📥

Rupa Framework provides a reactive asynchronous task queue system via the `rupa-queue` crate. This allows applications to offload heavy operations to background workers while maintaining real-time UI feedback.

---

## 🏗️ Core Concepts

The queue system is built on three main pillars:

### 1. `AsyncTask` (The Port)
A trait that defines the work to be done. Any background operation must implement this trait.

```rust
#[async_trait]
impl AsyncTask for MyHeavyTask {
    fn name(&self) -> &str { "Image Optimization" }
    async fn run(&self) -> Result<(), Error> {
        // Heavy logic here...
        Ok(())
    }
}
```

### 2. `AsyncQueue` (The Orchestrator)
Manages a pool of tasks with configurable concurrency. It uses `tokio` for execution and `rupa-signals` to report state.

### 3. `TaskHandle` (The Reactive Bridge)
Provides real-time monitoring of a task's lifecycle (`Pending`, `Running`, `Completed`, `Failed`).

---

## ⚡ Usage Patterns

### Initializing a Queue
Queues are typically registered as global services via `rupa-context`.

```rust
let queue = AsyncQueue::new(4); // Allow 4 concurrent tasks
provide_context(queue);
```

### Pushing Tasks
```rust
let queue = use_context::<Arc<AsyncQueue>>().unwrap();
queue.push(MyHeavyTask::new());
```

### Monitoring Progress in UI
Because the queue is reactive, you can easily render a task list or progress bar.

```rust
let tasks = queue.tasks(); // Signal<Vec<TaskHandle>>

ForEach::new(tasks, |handle| {
    VStack::new()
        .child(Text::new(handle.name))
        .child(ProgressBar::new(handle.progress))
})
```

---

## 🚀 Benefits of Hexagonal Queues

1.  **Non-blocking UI**: Heavy I/O or CPU tasks never freeze the main event loop.
2.  **Concurrency Control**: Prevents system exhaustion by limiting active workers.
3.  **Observability**: Built-in reactive signals make it trivial to build "Task Manager" style UIs.
