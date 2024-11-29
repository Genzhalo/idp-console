use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    app::services::submission::SubmissionService,
    extra::{auth_data::AuthData, json_input::JsonInput},
    AppState,
};

pub fn build_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/submissions/:sub_id/status", post(udpate_status))
        .route("/api/submissions/:sub_id", delete(delete_sub))
}

#[derive(Debug, Deserialize)]
struct UpdateSubStatusBody {
    status: String,
}

async fn udpate_status(
    Path(sub_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
    JsonInput(body): JsonInput<UpdateSubStatusBody>,
) -> Response {
    let service = SubmissionService::new(
        &state.config,
        state.db.submissions.as_ref(),
        state.db.users.as_ref(),
        state.db.forms.as_ref(),
        state.db.respondents.as_ref(),
        &auth.token,
    );

    match service.status(&sub_id, &body.status).await {
        Ok(()) => (StatusCode::OK, Json(json!({ "data": {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn delete_sub(
    Path(sub_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> Response {
    let service = SubmissionService::new(
        &state.config,
        state.db.submissions.as_ref(),
        state.db.users.as_ref(),
        state.db.forms.as_ref(),
        state.db.respondents.as_ref(),
        &auth.token,
    );
    match service.delete(&sub_id).await {
        Ok(()) => (StatusCode::OK, Json(json!({ "data": {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}
