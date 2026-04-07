use std::sync::{Arc, Mutex};

use axum::{extract::State, routing::get, Json, Router};
use provena_kernel::{Kernel, KernelHealth};

/// Shared API state. Holds a live reference to the kernel so the health
/// endpoint always reflects real-time registration state rather than a
/// startup snapshot.
#[derive(Clone)]
pub struct ApiState {
    pub kernel: Arc<Mutex<Kernel>>,
}

/// Build the Axum router with a live kernel reference.
///
/// The caller is responsible for constructing the `Arc<Mutex<Kernel>>` and
/// registering any plugins before (or after) calling this function.
pub fn build_router(kernel: Arc<Mutex<Kernel>>) -> Router {
    Router::new()
        .route("/health", get(health_handler))
        .with_state(ApiState { kernel })
}

async fn health_handler(State(state): State<ApiState>) -> Json<KernelHealth> {
    let health = state
        .kernel
        .lock()
        .map(|k| k.health())
        .unwrap_or_else(|_| KernelHealth {
            registered_plugins: 0,
            registered_capabilities: 0,
            active_capabilities: 0,
            standby_capabilities: 0,
        });

    Json(health)
}
