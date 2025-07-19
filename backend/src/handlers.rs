use crate::models::{AppState, Login, User, UserInDB};
use axum::{
    extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use validator::Validate;


pub fn create_router(app_state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/register", post(register_handler))
        .route("/api/login", post(login_handler))
        .layer(cors)
        .with_state(app_state)
}

async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(request): Json<User>,
) -> impl IntoResponse {
    if let Err(e) = request.validate() {
        let message = if e.field_errors().contains_key("password") {
            "Invalid password. Password must be at least 8 characters and include upper/lower case, number, and special character"
        } else {
            "Invalid email address"
        };
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({ "error": message })),
        );
    }

    if let Ok(Some(_)) = sqlx::query("SELECT email FROM users WHERE email = ?")
        .bind(&request.email)
        .fetch_optional(&state.db)
        .await
    {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::json!({"error": "Username already exists"})),
        );
    }

    let hashed_password =
        match tokio::task::spawn_blocking(move || hash(request.password, DEFAULT_COST)).await {
            Ok(Ok(h)) => h,
            _ => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": "Failed to hash password"})),
                )
            }
        };

    match sqlx::query!(
        "INSERT INTO users (email, password) VALUES (?, ?)",
        request.email,
        hashed_password
    )
    .execute(&state.db)
    .await
    {
        Ok(_) => (
            StatusCode::CREATED,
            Json(serde_json::json!({"message": "Account created successfully"})),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "An internal server error occurred."})),
        ),
    }

}

async fn login_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<Login>,
) -> impl IntoResponse {
    let user = match sqlx::query_as!(
        UserInDB,
        "SELECT email, password FROM users WHERE email = ?",
        payload.email
    )
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(user)) => user,
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({"error": "Invalid username or password"})),
            );
        }
    };

    let password_is_valid = tokio::task::spawn_blocking(move || verify(&payload.password, &user.password))
        .await
        .unwrap_or(Ok(false))
        .unwrap_or(false);

    if password_is_valid {
        (
            StatusCode::OK,
            Json(serde_json::json!({"message": "Logged in successfully"})),
        )
    } else {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "Invalid username or password"})),
        )
    }

}
