use serde::{Deserialize, Serialize};

use super::Entity;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sample {
    pub id: i64,
    pub name: String,
}

impl Sample {
    pub fn with_name(name: String) -> Self {
        Sample {
            id: i64::default(),
            name,
        }
    }

    pub fn new(id: i64, name: String) -> Self {
        Sample { id, name }
    }
}

impl Entity for Sample{
    type ID = i64;
    
    fn get_id(&self) -> &Self::ID{
        &self.id
    }
}