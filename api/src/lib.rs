pub mod error;
mod extractor;
mod hashing;
mod util;

mod routes;

pub type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

use std::{env, net::SocketAddr, sync::Arc};

use axum::{Json, Router, Server};
use axum_extra::routing::SpaRouter;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use db::{get_client, prisma::PrismaClient};
use error::{AppError, MainError};
use routes::{article, comment, profile, user};

#[derive(Clone)]
pub struct AppState {
    pub client: Arc<PrismaClient>,
    pub hmac_key: Arc<String>,
}

pub async fn run() -> Result<(), MainError> {
    //dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "realworld-axum-prisma=debug,info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client = Arc::new(get_client().await?);
    let hmac_key = Arc::new(env::var("HMAC_KEY")?);

    let state = AppState { client, hmac_key };

    let spa = SpaRouter::new("/assets", "./frontend/dist").index_file("index.html");
    let app = app(state)
        .merge(spa);

    let addr: SocketAddr = "[::]:8080".parse()?;

    info!("Server listening on {}", &addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|_| MainError::BindingError)?;

    Ok(())
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .merge(article::create_routes())
        .merge(comment::create_routes())
        .merge(profile::create_routes())
        .merge(user::create_routes())
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any))
        .with_state(state)
}
