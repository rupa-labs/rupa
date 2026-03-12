//! # Rupa Auth 🔐
//!
//! Identity and Session Management for the Rupa Framework. 
//! Defines the foundational traits (Ports) for authentication and access control.

pub mod identity;
pub mod session;
pub mod rbac;
pub mod teams;
pub mod utils;

use std::sync::Arc;
use rupa_signals::Signal;
pub use identity::User;
pub use session::Session;
pub use rbac::*;
pub use teams::*;
use rupa_base::Error;
use async_trait::async_trait;

/// The primary Port for Authentication services.
///
/// This trait must be implemented by concrete Showroom Adapters (e.g., Supabase, Auth0, OAuth).
#[async_trait]
pub trait Service: Send + Sync {
    /// Attempts to authenticate a user with the provided credentials.
    async fn login(&self, credentials: serde_json::Value) -> Result<(User, Session), Error>;
    /// Invalidates the current session.
    async fn logout(&self) -> Result<(), Error>;
    /// Refreshes the current session token.
    async fn refresh_session(&self) -> Result<Session, Error>;
    /// Returns the currently authenticated user, if any.
    async fn get_current_user(&self) -> Result<Option<User>, Error>;
}

/// A reactive container for the current Authentication state.
///
/// Used by UI components to react to login/logout events.
#[derive(Clone)]
pub struct Status {
    /// The currently authenticated user.
    pub user: Signal<Option<User>>,
    /// The active session metadata.
    pub session: Signal<Option<Session>>,
    /// Whether an authentication request is in progress.
    pub is_loading: Signal<bool>,
}

impl Status {
    /// Creates a new, empty authentication status.
    pub fn new() -> Self {
        Self {
            user: Signal::new(None),
            session: Signal::new(None),
            is_loading: Signal::new(false),
        }
    }

    /// Helper: Checks if there is an active session.
    pub fn is_authenticated(&self) -> bool {
        self.session.get().is_some()
    }
}

/// A thread-safe handle to an Authentication Service.
pub type Port = Arc<dyn Service>;

/// A mock implementation of the Auth Service for TDD and headless testing.
pub struct MockService {
    pub user: User,
    pub session: Session,
}

#[async_trait]
impl Service for MockService {
    async fn login(&self, _credentials: serde_json::Value) -> Result<(User, Session), Error> {
        Ok((self.user.clone(), self.session.clone()))
    }

    async fn logout(&self) -> Result<(), Error> {
        Ok(())
    }

    async fn refresh_session(&self) -> Result<Session, Error> {
        Ok(self.session.clone())
    }

    async fn get_current_user(&self) -> Result<Option<User>, Error> {
        Ok(Some(self.user.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_auth_flow() {
        let user = User::default();
        let session = Session::default();
        let service = MockService { user: user.clone(), session: session.clone() };
        
        let (logged_user, _sess) = service.login(serde_json::json!({})).await.unwrap();
        assert_eq!(logged_user, user);
    }
}
