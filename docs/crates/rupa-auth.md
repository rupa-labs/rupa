# `rupa-auth` 🔐

**The Identity & Session Port.** Agnostic authentication and authorization services.

## 🛠️ Key Features

- **`Service`**: Port for Login, Logout, and Session management.
- **`Status`**: Reactive container for the current User and Session state.
- **`Provider`**: Port for Role-Based Access Control (RBAC).
- **`User` & `Session`**: Standardized data models for identity.

## 🚀 Usage

```rust
use rupa_auth::{Status, Service};

// 1. Monitor auth state in UI
let auth = use_context::<Status>().unwrap();

Effect::new(move || {
    if auth.is_authenticated() {
        println!("Welcome back, {}!", auth.user.get().unwrap().username);
    }
});

// 2. Perform actions via Port
let service = use_context::<AuthPort>().unwrap();
service.login(credentials).await;
```
