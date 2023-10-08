use axum::{
    extract,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::models::participant::Participant;

pub async fn get_all_participants(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
) -> Result<Json<Vec<Participant>>, StatusCode> {
    let res = sqlx::query_as::<_, Participant>(
        r#"
        SELECT * FROM participants
        WHERE match_id = $1
        "#,
    )
    .bind(&id)
    .fetch_all(&pool)
    .await;

    match res {
        Ok(participants) => Ok(Json(participants)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn add_participant(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
    Json(payload): Json<Participant>,
) -> Result<(StatusCode, Json<Participant>), StatusCode> {
    let participant = Participant::new(id, payload.user_id);

    let res = sqlx::query(
        r#"
        INSERT INTO participants (leagueid, userid)
        VALUES ($1, $2)
        "#,
    )
    .bind(&participant.league_id)
    .bind(&participant.user_id)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(participant))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn remove_participant(
    extract::State(pool): extract::State<PgPool>,
    extract::Path((leagueid, userid)): extract::Path<(uuid::Uuid, uuid::Uuid)>,
) -> Result<StatusCode, StatusCode> {
    let res = sqlx::query(
        r#"
        DELETE FROM participants
        WHERE leagueid = $1 AND userid = $2
        "#,
    )
    .bind(&leagueid)
    .bind(&userid)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


