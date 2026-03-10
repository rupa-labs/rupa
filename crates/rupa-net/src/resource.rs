use rupa_signals::Signal;
use rupa_support::Error;

/// A reactive state machine for asynchronous data fetching.
#[derive(Clone, Debug, PartialEq)]
pub enum Resource<T> {
    Uninitialized,
    Loading,
    Ready(T),
    Error(Error),
}

impl<T: Clone + Send + Sync + 'static> Resource<T> {
    pub fn new_signal() -> Signal<Self> {
        Signal::new(Resource::Uninitialized)
    }
}
