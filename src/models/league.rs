use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct League {
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateLeague {
    pub name: String,
}

impl League {
    pub fn new(name: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            name,
        }
    }
}