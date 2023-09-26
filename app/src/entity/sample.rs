use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Sample{
    pub id: i64,
    pub name: String
}
