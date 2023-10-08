use axum::{
    extract::{Path, self},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;

use crate::models::user::{User, NewUser};

pub async fn get_all_users(
    extract::State(pool): extract::State<PgPool>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let res = sqlx::query_as::<_, User>(
        r#"
        SELECT * FROM users
        "#,
    )
    .fetch_all(&pool)
    .await;

    match res {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn create_user(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<NewUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let user = User::new(payload.name);

    let res = sqlx::query(
        r#"
        INSERT INTO users (id, name)
        VALUES ($1, $2)
        "#,
    )
    .bind(&user.id)
    .bind(&user.name)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(user))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_user(
    extract::State(pool): extract::State<PgPool>,
    Path(id): Path<uuid::Uuid>,
    Json(payload): Json<NewUser>,
) -> Result<StatusCode, StatusCode> {
    let res = sqlx::query(
        r#"
        UPDATE users
        SET name = $1
        WHERE id = $2
        "#,
    )
    .bind(&payload.name)
    .bind(&id)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_user(
    extract::State(pool): extract::State<PgPool>,
    Path(id): Path<uuid::Uuid>,
) -> Result<StatusCode, StatusCode> {
    let res = sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#,
    )
    .bind(&id)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
