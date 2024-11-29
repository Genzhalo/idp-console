use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, patch, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    app::{
        entities::form::status::FormStatus,
        services::{
            form::{CreateFromData, FormService, UpdateFromData},
            submission::{GetQuery, SubmissionService},
        },
    },
    extra::{auth_data::AuthData, json_input::JsonInput},
    AppState,
};

pub fn build_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/forms", get(get_forms))
        .route("/api/forms", post(create_form))
        .route("/api/forms/:form_id", get(get_form))
        .route("/api/forms/:form_id", patch(update_form))
        .route("/api/forms/:form_id", delete(delete_form))
        .route("/api/forms/:form_id/open", post(open_form))
        .route("/api/forms/:form_id/close", post(close_form))
        .route("/api/forms/:form_id/submissions", get(get_submissions))
        .route("/api/forms/:form_id/submissions", post(create_submission))
}

async fn get_forms(State(state): State<Arc<AppState>>, auth: AuthData) -> Response {
    let service = FormService::new(
        &state.config,
        state.db.forms.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );
    match service.get().await {
        Ok(forms) => (StatusCode::OK, Json(json!({"data": forms}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn create_form(
    State(state): State<Arc<AppState>>,
    auth: AuthData,
    JsonInput(body): JsonInput<CreateFromData>,
) -> Response {
    let service = FormService::new(
        &state.config,
        state.db.forms.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );

    match service.create(body).await {
        Ok(id) => (StatusCode::OK, Json(json!({"data": id}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn get_form(
    Path(form_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> Response {
    let service = FormService::new(
        &state.config,
        state.db.forms.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );

    match service.get_by_id(&form_id).await {
        Ok(forms) => (StatusCode::OK, Json(json!({"data": forms}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn update_form(
    Path(form_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
    JsonInput(body): JsonInput<UpdateFromData>,
) -> Response {
    let service = FormService::new(
        &state.config,
        state.db.forms.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );

    match service.update(form_id.clone(), body).await {
        Ok(()) => (StatusCode::OK, Json(json!({ "data": { "id": form_id } }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn delete_form(
    Path(form_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> Response {
    let service = FormService::new(
        &state.config,
        state.db.forms.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );
    match service.delete(&form_id).await {
        Ok(()) => (StatusCode::OK, Json(json!({ "data": {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn open_form(
    Path(form_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> Response {
    let service = FormService::new(
        &state.config,
        state.db.forms.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );
    match service.status(form_id, FormStatus::Open).await {
        Ok(()) => (StatusCode::OK, Json(json!({ "data": {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn close_form(
    Path(form_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
) -> Response {
    let service = FormService::new(
        &state.config,
        state.db.forms.as_ref(),
        state.db.users.as_ref(),
        &auth.token,
    );
    match service.status(form_id, FormStatus::Close).await {
        Ok(()) => (StatusCode::OK, Json(json!({ "data": {} }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

async fn get_submissions(
    Path(form_id): Path<String>,
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
    let query = GetQuery {
        form_id: Some(form_id),
        respondent_id: None,
    };
    match service.get(query).await {
        Ok(data) => (StatusCode::OK, Json(json!({ "data": data}))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}

#[derive(Debug, Deserialize)]
struct CreateSubmissionBody {
    #[serde(rename = "respondentId")]
    respondent_id: String,
}
async fn create_submission(
    Path(form_id): Path<String>,
    State(state): State<Arc<AppState>>,
    auth: AuthData,
    JsonInput(body): JsonInput<CreateSubmissionBody>,
) -> Response {
    let service = SubmissionService::new(
        &state.config,
        state.db.submissions.as_ref(),
        state.db.users.as_ref(),
        state.db.forms.as_ref(),
        state.db.respondents.as_ref(),
        &auth.token,
    );
    match service.create(&form_id, &body.respondent_id).await {
        Ok(data) => (StatusCode::OK, Json(json!({ "data": data }))).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({ "data":  err }))).into_response(),
    }
}
