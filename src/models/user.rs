use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
        }
    }
}