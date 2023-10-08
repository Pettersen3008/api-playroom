use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Matches {
    pub id: uuid::Uuid,
    pub league_id: uuid::Uuid,
    pub home_user_id: uuid::Uuid,
    pub away_user_id: uuid::Uuid,
    pub home_score: i32,
    pub away_score: i32,
    pub date_scheduled: DateTime<Utc>,
    pub date_played: Option<DateTime<Utc>>,
}

impl Matches {
    pub fn new(league_id: uuid::Uuid, home_user_id: uuid::Uuid, away_user_id: uuid::Uuid, home_score: i32, away_score: i32, date_scheduled: DateTime<Utc>, date_played: Option<DateTime<Utc>>) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            league_id,
            home_user_id,
            away_user_id,
            home_score,
            away_score,
            date_scheduled,
            date_played,
        }
    }
}
