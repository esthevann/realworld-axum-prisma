#![feature(let_chains)]


mod error;
mod extractor;
mod hashing;
mod util;

mod db;
mod routes;

pub type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

use std::{env, net::SocketAddr, sync::Arc};

use axum::{Json, Router, Server};
use tower_http::{trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info};

use util::MergeRouter;
use error::{AppError, MainError};
use db::prisma::PrismaClient;
use routes::{article, profile, user, comment};



#[derive(Clone)]
pub struct AppState {
    client: Arc<PrismaClient>,
    hmac_key: Arc<String>,
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
    dotenvy::dotenv().ok();
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "realworld-axum-prisma=debug,info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client = Arc::new(PrismaClient::_builder().build().await?);
    let hmac_key = Arc::new(env::var("HMAC_KEY")?);

    let state = Arc::new(AppState { client, hmac_key });
    
    let app = Router::with_state_arc(state)
        .merge_router(user::create_routes)
        .merge_router(article::create_routes)
        .merge_router(profile::create_routes)
        .merge_router(comment::create_routes)
        .layer(TraceLayer::new_for_http());


    let addr: SocketAddr = "0.0.0.0:5000".parse()?;

    info!("Server listening on {}", &addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|_| MainError::BindingError)?;

    Ok(())
}
