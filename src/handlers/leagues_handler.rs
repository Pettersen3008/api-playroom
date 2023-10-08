use axum::{
    extract,
    http,
};
use crate::models::league::{League, CreateLeague};
use sqlx::PgPool;

pub async fn get_all_leagues(
    extract::State(pool): extract::State<PgPool>,
) -> Result<axum::Json<Vec<League>>, http::StatusCode> {
    let res = sqlx::query_as::<_, League>("SELECT * FROM leagues")
    .fetch_all(&pool)
    .await;

    match res {
        Ok(leagues) => Ok(axum::Json(leagues)),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_league(
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateLeague>,
) -> Result<(http::StatusCode, axum::Json<League>), http::StatusCode> {
    let league = League::new(payload.name);

    let res = sqlx::query(
        r#"
        INSERT INTO leagues (id, name)
        VALUES ($1, $2)
        "#,
    )
    .bind(&league.id)
    .bind(&league.name)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, axum::Json(league))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_league(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<String>,
    axum::Json(payload): axum::Json<CreateLeague>,
) -> Result<(http::StatusCode, axum::Json<League>), http::StatusCode> {
    let res = sqlx::query_as::<_, League>(
        r#" UPDATE leagues SET name = $1 WHERE id = $2 RETURNING * "#,
    )
    .bind(&payload.name)
    .bind(&id)
    .fetch_one(&pool)
    .await;

    match res {
        Ok(league) => Ok((http::StatusCode::OK, axum::Json(league))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_league(
    extract::State(pool): extract::State<PgPool>,
    extract::Path(id): extract::Path<String>,
) -> Result<http::StatusCode, http::StatusCode> {
    let res = sqlx::query(
        r#"
        DELETE FROM leagues WHERE id = $1
        "#,
    )
    .bind(&id)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok(http::StatusCode::NO_CONTENT),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}