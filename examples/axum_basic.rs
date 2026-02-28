use anyhow::{Error, Result};
use axum::{
    Extension, Json,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use urocyon::{RequestContext, ServerBuilder};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
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

async fn create_user(
    Extension(context): Extension<RequestContext>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    context.log_info("Hello!");

    (StatusCode::CREATED, Json(user))
}
