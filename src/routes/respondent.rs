use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Json, Router,
};

use serde_json::json;

use crate::{
    app::services::{
        respondent::{
            self, create_data::CreateData, update_data::UpdateData, MergeData, RespondentService,
        },
        submission::{self, SubmissionService},
    },
    extra::{auth_data::AuthData, json_input::JsonInput},
    AppState,
};

pub fn build_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/respondents", get(get_respondents))
        .route("/api/respondents", post(create_respondent))
        .route("/api/respondents/:respondent_id", get(get_respondent))
        .route("/api/respondents/:respondent_id", patch(update_respondent))
        .route("/api/respondents/:respondent_id", delete(delete_respondent))
        .route(
            "/api/respondents/:respondent_id/merge",
            post(merge_respondent),
        )
        .route(
            "/api/respondents/:respondent_id/submissions",
            get(get_submissions),
        )
}

async fn get_respondents(
    Query(query): Query<respondent::GetQuery>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> Response {
    let service = RespondentService::new(
        &state.config,
        state.db.respondents.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );
    match service.get(query).await {
        Ok(data) => (StatusCode::OK, Json(json!({"data": data}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn create_respondent(
    State(state): State<Arc<AppState>>,
    auth: AuthData,
    JsonInput(body): JsonInput<CreateData>,
) -> Response {
    let service = RespondentService::new(
        &state.config,
        state.db.respondents.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );

    match service.create(&body).await {
        Ok(id) => (StatusCode::OK, Json(json!({"data": id}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn update_respondent(
    Path(respondent_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
    JsonInput(body): JsonInput<UpdateData>,
) -> Response {
    let service = RespondentService::new(
        &state.config,
        state.db.respondents.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );

    match service.update(respondent_id, &body).await {
        Ok(data) => (StatusCode::OK, Json(json!({"data": data}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn get_respondent(
    Path(respondent_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> Response {
    let service = RespondentService::new(
        &state.config,
        state.db.respondents.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );

    match service.get_by_id(&respondent_id).await {
        Ok(data) => (StatusCode::OK, Json(json!({"data": data}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn delete_respondent(
    Path(respondent_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> Response {
    let service = RespondentService::new(
        &state.config,
        state.db.respondents.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );

    match service.delete(respondent_id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"data": {}}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn merge_respondent(
    Path(respondent_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
    JsonInput(body): JsonInput<MergeData>,
) -> Response {
    let service = RespondentService::new(
        &state.config,
        state.db.respondents.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );

    match service.merge(&respondent_id, &body).await {
        Ok(_) => (StatusCode::OK, Json(json!({"data": {}}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn get_submissions(
    Path(respondent_id): Path<String>,
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
    let query = submission::GetQuery {
        form_id: None,
        respondent_id: Some(respondent_id),
    };
    match service.get(query).await {
        Ok(data) => (StatusCode::OK, Json(json!({ "data": data}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}
