use axum::{Router, routing::get};

use crate::AppState;

pub fn create_route(state: &AppState) -> Router<AppState> {
    Router::with_state(state.clone())
        .route("/api/article", get(handle_list_article))
        
}

async fn handle_list_article() -> String {
    "I'm an endpoint".to_owned()
}