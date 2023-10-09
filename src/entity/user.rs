use serde::{Deserialize, Serialize};

use super::Entity;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn with_name(name: String) -> Self {
        User {
            id: i64::default(),
            name,
            email: "abc@d.e".to_owned(),
        }
    }

    pub fn new(id: i64, name: String) -> Self {
        User {
            id,
            name,
            email: "abc@d.e".to_owned(),
        }
    }
}

impl Entity for User {
    type ID = i64;

    fn get_id(&self) -> &Self::ID {
        &self.id
    }
}
