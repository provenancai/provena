use axum::{extract::State, routing::get, Json, Router};
use provena_kernel::KernelHealth;

#[derive(Clone)]
struct ApiState {
    health: KernelHealth,
}

pub fn build_router(health: KernelHealth) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .with_state(ApiState { health })
}

async fn health_handler(State(state): State<ApiState>) -> Json<KernelHealth> {
    Json(state.health)
}