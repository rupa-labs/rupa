use rupa_base::Error;
use serde::{Serialize, de::DeserializeOwned};
use std::any::Any;
use std::sync::Arc;
use rupa_broadcast::{Bus as BroadcastBus, Broadcaster};

/// A trait for any data structure that represents an Artisan Action.
pub trait Action: Serialize + DeserializeOwned + Any + Send + Sync + Clone {}

/// The logic that executes a specific Action.
pub trait Handler<A: Action>: Send + Sync {
    fn handle(&self, action: A) -> Result<(), Error>;
}

/// A reactive Action Bus for dispatching commands.
pub struct Bus {
    inner: &'static BroadcastBus,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            inner: BroadcastBus::global(),
        }
    }

    /// Dispatches an action to all registered handlers.
    pub async fn dispatch<A: Action>(&self, action: A) {
        let channel = self.inner.channel::<A>(std::any::type_name::<A>());
        channel.dispatch(action).await;
    }

    /// Subscribes a handler to a specific action type.
    pub fn subscribe<A: Action>(&self, handler: Arc<dyn Handler<A>>) {
        let channel = self.inner.channel::<A>(std::any::type_name::<A>());
        channel.subscribe(Arc::new(move |action| {
            let _ = handler.handle(action);
        }));
    }
}

/// A type-erased wrapper for action handlers.
pub trait BoxedHandler: Send + Sync {
    fn handle_json(&self, json_payload: &str) -> Result<(), Error>;
}

pub struct GenericHandler<A: Action, H: Handler<A>> {
    pub handler: H,
    _marker: std::marker::PhantomData<A>,
}

impl<A: Action, H: Handler<A>> GenericHandler<A, H> {
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<A: Action, H: Handler<A>> BoxedHandler for GenericHandler<A, H> {
    fn handle_json(&self, json_payload: &str) -> Result<(), Error> {
        let action: A = serde_json::from_str(json_payload)
            .map_err(|e| Error::Platform(format!("Failed to deserialize action payload: {}", e)))?;
        self.handler.handle(action)
    }
}
