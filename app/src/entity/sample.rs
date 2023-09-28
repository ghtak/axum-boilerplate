use serde::{Deserialize, Serialize};

use super::Entity;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sample {
    pub id: i64,
    pub name: String,
}

impl Sample {
    pub fn from_name(name: String) -> Self {
        Sample {
            id: Self::NEW_ENTITY_ID,
            name,
        }
    }

    pub fn new(id: i64, name: String) -> Self {
        Sample { id, name }
    }
}

impl Entity for Sample {
    fn is_new(&self) -> bool {
        self.id == Self::NEW_ENTITY_ID
    }
}
