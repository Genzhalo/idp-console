use axum::{
    async_trait, body::Body, extract::{rejection::JsonRejection, FromRequest}, http::{Request, StatusCode}
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonInput<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for JsonInput<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        let req = Request::from_parts(parts, body);

        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let payload = json!({
                    "data": {"message": rejection.body_text()},
                });

                Err((rejection.status(), axum::Json(payload)))
            }
        }
    }
}