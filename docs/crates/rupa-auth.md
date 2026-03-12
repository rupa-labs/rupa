# `rupa-auth` 🔐

**The Identity Port.** This crate defines the foundational **Atoms** for authentication and authorization, providing a unified contract for identity services.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Strict type boundaries for `User` and `Session` data prevent field-injection vulnerabilities.
    - **Sustain (S2)**: Decoupled `Service` port allows for zero-rework backend swaps.
    - **Scalable (S3)**: Reactive `Status` container ensures UI updates are surgical during auth state changes.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Service`** | The Auth Port. | Contract for Login, Logout, and Session refresh. |
| **`Status`** | Reactive Auth State. | Wraps User and Session in reactive Signals. |
| **`MockService`** | Testing Implementation. | Pre-configured service for TDD and headless tests. |
| **`User` & `Session`** | Identity Models. | Serializable, industry-standard field mapping. |

## 🚀 Usage

```rust
use rupa_auth::{Status, MockService, Service};

// 1. Monitor auth state in UI
let auth = Status::new();

Effect::new({
    let auth = auth.clone();
    move || {
        if auth.is_authenticated() {
            println!("Welcome back, {}!", auth.user.get().unwrap().name);
        }
    }
});

// 2. Perform actions via Port (Implemented by Showroom)
let service: Arc<dyn Service> = Arc::new(MockService::default());
service.login(serde_json::json!({"token": "xyz"})).await?;
```

## 🧪 Testing & Reliability
- **Mock Implementation**: Includes a built-in `MockService` allowing developers to build and test UI flows without a real auth server.
- **TDD Support**: Verified authentication flows (login/refresh/logout) in headless environments.
- **Zero-Knowledge**: The core Atom does not know about cookies, JWTs, or OAuth; it only knows the *Port* contract.
