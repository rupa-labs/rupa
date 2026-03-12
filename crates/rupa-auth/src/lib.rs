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
#[async_trait]
pub trait Service: Send + Sync {
    async fn login(&self, credentials: serde_json::Value) -> Result<(User, Session), Error>;
    async fn logout(&self) -> Result<(), Error>;
    async fn refresh_session(&self) -> Result<Session, Error>;
    async fn get_current_user(&self) -> Result<Option<User>, Error>;
}

/// A reactive container for the current Authentication state.
#[derive(Clone)]
pub struct Status {
    pub user: Signal<Option<User>>,
    pub session: Signal<Option<Session>>,
    pub is_loading: Signal<bool>,
}

impl Status {
    pub fn new() -> Self {
        Self {
            user: Signal::new(None),
            session: Signal::new(None),
            is_loading: Signal::new(false),
        }
    }

    pub fn is_authenticated(&self) -> bool {
        self.session.get().is_some()
    }
}

pub type Port = Arc<dyn Service>;
