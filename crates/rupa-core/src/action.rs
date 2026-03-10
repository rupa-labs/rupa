use rupa_support::Error;
use serde::{Serialize, de::DeserializeOwned};
use std::any::Any;

/// A trait for any data structure that represents an Artisan Action.
pub trait Action: Serialize + DeserializeOwned + Any + Send + Sync {}

/// The logic that executes a specific Action.
pub trait ActionHandler<A: Action>: Send + Sync {
    fn handle(&self, action: A) -> Result<(), Error>;
}

/// A type-erased wrapper for action handlers to be stored in a Registry.
pub trait BoxedActionHandler: Send + Sync {
    fn handle_json(&self, json_payload: &str) -> Result<(), Error>;
}

impl<A: Action, H: ActionHandler<A>> BoxedActionHandler for H {
    fn handle_json(&self, json_payload: &str) -> Result<(), Error> {
        let action: A = serde_json::from_str(json_payload)
            .map_err(|e| Error::Platform(format!("Failed to deserialize action payload: {}", e)))?;
        self.handle(action)
    }
}
