use crate::handlers::create_router;
use crate::models::AppState;
use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;
use std::env;

mod handlers;
mod models;

#[tokio::main]
async fn main() {

    dotenvy::dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("DB Url is :{}",database_url);

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool.");

    println!("DB connected successfully");

    let app_state = Arc::new(AppState { db: pool });
    let app = create_router(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server listening on 0.0.0.0:3000");
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
