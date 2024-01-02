mod demo;
mod api;
use axum::*;
use sea_orm::{Database, DatabaseConnection};
use std::env;


#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[tokio::main]
pub async fn start() {

    tracing_subscriber::fmt::init();

    // tracing_subscriber::registry()
    //     .with(
    //         tracing_subscriber::EnvFilter::try_from_default_env()
    //             .unwrap_or_else(|_| "example_low_level_openssl=debug".into()),
    //     )
    //     .with(tracing_subscriber::fmt::layer())
    //     .init();

    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    let state = AppState { conn };

    let app = api::register_api(Router::new()).with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
