use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde_json::{json, Value};
use std::{cell::RefCell, rc::Rc, sync::Arc};
use tower_http::cors::CorsLayer;

use coffee_brewing_tracker::services::BrewingService;
use coffee_brewing_tracker::models::{NewCoffeeBean, NewGrinder, NewBrewingSession};

type AppState = Arc<BrewingService>;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres@localhost:5433/coffee_tracker".to_string());
    
    let service = BrewingService::new(&database_url)
        .expect("Failed to connect to database");
    
    let app_state = Arc::new(service);
    
    let app = Router::new()
        .route("/api/beans", get(get_beans).post(create_bean))
        .route("/api/grinders", get(get_grinders).post(create_grinder))
        .route("/api/sessions", get(get_sessions).post(create_session))
        .route("/api/sessions/:id", get(get_session))
        .layer(CorsLayer::permissive())
        .with_state(app_state);
    
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running on http://127.0.0.1:3000");
    println!("📊 Database: {}", database_url);
    
    axum::serve(listener, app).await.unwrap();
}

async fn get_beans(State(service): State<AppState>) -> Result<Json<Value>, StatusCode> {
    service.get_beans()
        .map(|beans| Json(json!(beans)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_bean(
    State(service): State<AppState>,
    Json(bean): Json<NewCoffeeBean>,
) -> Result<Json<Value>, StatusCode> {
    service.add_bean(bean)
        .map(|bean| Json(json!(bean)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_grinders(State(service): State<AppState>) -> Result<Json<Value>, StatusCode> {
    service.get_grinders()
        .map(|grinders| Json(json!(grinders)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_grinder(
    State(service): State<AppState>,
    Json(grinder): Json<NewGrinder>,
) -> Result<Json<Value>, StatusCode> {
    service.add_grinder(grinder)
        .map(|grinder| Json(json!(grinder)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_sessions(State(service): State<AppState>) -> Result<Json<Value>, StatusCode> {
    service.get_sessions(Default::default())
        .map(|sessions| Json(json!(sessions)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn create_session(
    State(service): State<AppState>,
    Json(session): Json<NewBrewingSession>,
) -> Result<Json<Value>, StatusCode> {
    service.create_session(session)
        .map(|session| Json(json!(session)))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn get_session(
    State(service): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, StatusCode> {
    service.get_session_by_id(id)
        .map(|session| Json(json!(session)))
        .map_err(|_| StatusCode::NOT_FOUND)
}
