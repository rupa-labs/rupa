use rupa_signals::Signal;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Session {
    pub token: String,
    pub expiry: u64,
}

pub struct AuthState {
    pub current_user: Signal<Option<super::identity::User>>,
    pub session: Signal<Option<Session>>,
}
