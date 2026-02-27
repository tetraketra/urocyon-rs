use axum::{
    Json,
    http::StatusCode,
    routing::{get, post},
};

use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

mod server;
use server::ServerBuilder;

refinery::embed_migrations!("migrations"); // TODO: Move to part of args/serve setup.

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    ServerBuilder::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .build_and_serve()
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}
