use axum::{
    routing::{get, post, put, delete},
    http::StatusCode,
    Json, Router,
    extract::Path
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn get_users() -> (Json<User>) {
    let user = User {
        id: 1337,
        username: String::from("arya"),
    };
    (Json(user))
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}


async fn get_user_by_id (Path(id): Path <u64>) -> (Json<User>) {
    let user = User {
        id,
        username: String::from("arya"),
    };
    (Json(user))
}

async fn edit_user_by_id () -> (Json<User>) {
    let user = User {
        id: 1337,
        username: String::from("arya"),
    };
    (Json(user))
}

async fn delete_user_by_id () -> (Json<User>) {
    let user = User {
        id: 1337,
        username: String::from("arya"),
    };
    (Json(user))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", get(get_users))
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user_by_id))
        .route("/users/:id", put(edit_user_by_id))
        .route("/users/:id", delete(edit_user_by_id));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

