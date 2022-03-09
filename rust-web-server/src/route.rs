use axum::{Json, Router};
use axum::routing::{get, post};
use crate::{IntoResponse, StatusCode};
use serde::{Deserialize, Serialize};


pub fn route() -> Router {
    Router::new()
        .route("/", get(root))
        .route("user", post(create_user))
}

async fn root() -> &'static str{
    "hello world"
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1,
        user_name: payload.username
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
    struct User {
    id: u64,
    user_name: String,
}
