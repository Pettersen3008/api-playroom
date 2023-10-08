use axum::{
    extract,
    http,
};
use sqlx::PgPool;

use crate::models::matches::Matches;

pub async fn get_all_matches(
    extract::State(pool): extract::State<PgPool>
) -> Result<axum::Json<Vec<Matches>>, http::StatusCode> {
    let res = sqlx::query_as::<_, Matches>("SELECT * FROM matches")
    .fetch_all(&pool)
    .await;

    match res {
        Ok(matches) => Ok(axum::Json(matches)),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_match(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<String>
) -> Result<axum::Json<Matches>, http::StatusCode> {
    let res = sqlx::query_as::<_, Matches>(
        r#"
        SELECT * FROM matches
        WHERE id = $1
        "#,
    )
    .bind(&id)
    .fetch_one(&pool)
    .await;

    match res {
        Ok(matches) => Ok(axum::Json(matches)),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn record_match_result(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(leagueid): extract::Path<uuid::Uuid>,
    extract::Path(matchid): extract::Path<uuid::Uuid>,
    extract::Json(payload): extract::Json<Matches>,
) -> Result<http::StatusCode, http::StatusCode> {
    let res = sqlx::query(
        r#"
        UPDATE matches SET leagueId = $1, homeUserId = $2, awayUserId = $3, homeScore = $4, awayScore = $5, dateScheduled = $6, datePlayed = $7
        WHERE id = $8
        "#,
    )
    .bind(leagueid)
    .bind(payload.home_user_id)
    .bind(payload.away_user_id)
    .bind(payload.home_score)
    .bind(payload.away_score)
    .bind(payload.date_scheduled)
    .bind(payload.date_played)
    .bind(matchid)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok(http::StatusCode::OK),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn generate_match_schedule(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
) -> Result<http::StatusCode, http::StatusCode> {
    // TODO: Implement two new functions:
    let new_match = Matches::new(
        id,
        // randomize home and away not implemented yet.
        uuid::Uuid::new_v4(),
        // randomize home and away not implemented yet.
        uuid::Uuid::new_v4(),
        0,
        0,
        // randomize date based on the next week
        chrono::Utc::now() + chrono::Duration::weeks(1),
        // randomize date based on the next week
        None,
    );
    let res = sqlx::query(
        r#"
        INSERT INTO matches (leagueId, homeUserId, awayUserId, homeScore, awayScore, dateScheduled, datePlayed)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(&new_match.league_id)
    .bind(&new_match.home_user_id)
    .bind(&new_match.away_user_id)
    .bind(&new_match.home_score)
    .bind(&new_match.away_score)
    .bind(&new_match.date_scheduled)
    .bind(&new_match.date_played)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok(http::StatusCode::CREATED),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}