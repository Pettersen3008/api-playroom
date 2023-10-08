use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Participant {
    pub league_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
}

impl Participant {
    pub fn new(league_id: uuid::Uuid, user_id: uuid::Uuid) -> Self {
        Self {
            league_id,
            user_id,
        }
    }
}