# Auth & Session Port Architecture 🔐

The **Auth & Session Port** defines the protocols and enforcers for identity management, authentication, and access control.

---

## 1. Core Responsibilities

- **Protocols**: Agnostic interfaces for login methods (JWT, OAuth, Sessions).
- **Session Management**: Managing the lifecycle of authenticated states.
- **Enforcement (Gates & Policies)**: Providing the "Law" for checking permissions without knowing the underlying data models.

---

## 2. Technical Identity

- **Port Mandate**: It does not contain user models. It provides the **Trait Port** that Artisans or Showrooms implement.
- **Interface-First**: Enables swapping between local, cloud, or mock auth systems seamlessly.
