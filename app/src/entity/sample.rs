use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sample {
    pub id: i64,
    pub name: String,
}

impl Sample {
    pub fn from_name(name: String) -> Self {
        Sample {
            id: i64::default(),
            name,
        }
    }

    pub fn new(id: i64, name: String) -> Self {
        Sample { id, name }
    }
}
