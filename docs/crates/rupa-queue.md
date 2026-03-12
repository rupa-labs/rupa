# `rupa-queue` 📥

**The Background Task Port.** Reactive asynchronous task orchestration with concurrency control.

## 🛠️ Key Features

- **`Task`**: Port for defining background operations.
- **`Queue`**: Orchestrator for task execution and concurrency limiting.
- **`Handle`**: Reactive handle for tracking task progress and status.

## 🚀 Usage

```rust
use rupa_queue::{Queue, Task};

// 1. Create a Queue
let queue = Queue::new(4); // 4 concurrent workers

// 2. Push a task
queue.push(MyHeavyTask::new());

// 3. Monitor in UI
let tasks = queue.tasks(); // Signal<Vec<Handle>>
```
