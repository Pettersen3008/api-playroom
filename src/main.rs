use axum::routing::{Router, get, post, patch, delete};

mod handlers;
mod models;
mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let pool = database::establish_db_pool().await?;

    let app = Router::new()
        .route("/users", get(handlers::users_handler::get_all_users))
        .route("/users", post(handlers::users_handler::create_user))
        .route("/users/:id", patch(handlers::users_handler::update_user))
        .route("/users/:id", delete(handlers::users_handler::delete_user))
        .route("/leagues", get(handlers::leagues_handler::get_all_leagues))
        .route("/leagues", post(handlers::leagues_handler::create_league))
        .route("/leagues/:id", patch(handlers::leagues_handler::update_league))
        .route("/leagues/:id", delete(handlers::leagues_handler::delete_league))
        .route("/leagues/:id/participants", get(handlers::participants_handler::get_all_participants))
        .route("/leagues/:id/participants", post(handlers::participants_handler::add_participant))
        .route("/leagues/:id/participants/:user_id}", delete(handlers::participants_handler::remove_participant))
        .route("/leagues/:id/matches", get(handlers::matches_handler::get_all_matches))
        .route("/leagues/:id/matches/:match_id", get(handlers::matches_handler::get_match))
        .route("/leagues/:id/matches/:match_id", post(handlers::matches_handler::record_match_result))
        .route("/leagues/:id/matches/generate", get(handlers::matches_handler::generate_match_schedule))
        .with_state(pool);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
