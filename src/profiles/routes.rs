use axum::{Router, routing::get};

use crate::AppState;

pub fn create_route(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/api/profile/:username", get(handle_list_article))
        
}

async fn handle_list_article() -> String {
    "I'm an endpoint".to_owned()
}