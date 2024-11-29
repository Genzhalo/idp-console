use app::config::Config;
use axum::Router;
use db::DB;
use dotenv::dotenv;
use routes::{auth, form, respondent, submission};
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

mod app;
mod db;
mod extra;
mod routes;

pub struct AppState {
    db: DB,
    config: Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let jwt_secret_key = std::env::var("JWT_SECRET_KEY").expect("set JWT_SECRET_KEY env variable");
    let config = Config { jwt_secret_key };
    let db = DB::connect().await;
    db.init_default_user(&config).await;

    let app_state = Arc::new(AppState { db, config });
    let app = Router::new()
        .merge(auth::build_routes())
        .merge(form::build_routes())
        .merge(respondent::build_routes())
        .merge(submission::build_routes())
        .with_state(app_state)
        .nest_service("/assets", ServeDir::new("./dist/assets"))
        .fallback_service(ServeFile::new("./dist/index.html"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
