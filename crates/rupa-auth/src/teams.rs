use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub members: Vec<String>,
}

impl Team {
    pub fn new(id: impl Into<String>, name: impl Into<String>, owner_id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            owner_id: owner_id.into(),
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, user_id: impl Into<String>) {
        let id = user_id.into();
        if !self.members.contains(&id) {
            self.members.push(id);
        }
    }

    pub fn is_member(&self, user_id: &str) -> bool {
        self.owner_id == user_id || self.members.iter().any(|m| m == user_id)
    }
}
