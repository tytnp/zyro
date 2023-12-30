use axum::response::Html;
use axum::routing::get;
use axum::*;
use sea_orm::{Database, DatabaseConnection};
use std::env;

#[derive(Clone)]
struct AppState {
    _conn: DatabaseConnection,
}

#[tokio::main]
async fn start() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    let state = AppState { _conn: conn };
    let app = Router::new().route("/", get(index)).with_state(state);
    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn main() {
    start();
}

async fn index() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
