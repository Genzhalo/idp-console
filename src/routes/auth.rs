use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde_json::json;

use crate::{
    app::services::auth::{AuthService, LoginInputData},
    extra::{auth_data::AuthData, json_input::JsonInput},
    AppState,
};

pub fn build_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/auth/signin", post(sign_in))
        .route("/api/auth/signout", post(revoke_token))
}

async fn sign_in(
    State(state): State<Arc<AppState>>,
    JsonInput(body): JsonInput<LoginInputData>,
) -> Response {
    let service = AuthService::new(&state.config, state.db.users.as_ref());

    match service.login(body).await {
        Ok(token) => (StatusCode::OK, Json(json!({"data": token}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn revoke_token(State(state): State<Arc<AppState>>, auth: AuthData) -> Response {
    let service = AuthService::new(&state.config, state.db.users.as_ref());
    match service.revoke_token(&auth.token).await {
        Ok(_) => (StatusCode::OK, Json(json!({ "data":  {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}
