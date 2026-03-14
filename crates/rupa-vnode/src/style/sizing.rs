use serde::{Serialize, Deserialize};
use crate::types::Unit;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Sizing {
    pub width: Option<Unit>,
    pub height: Option<Unit>,
    pub min_width: Option<Unit>,
    pub min_height: Option<Unit>,
    pub max_width: Option<Unit>,
    pub max_height: Option<Unit>,
}
