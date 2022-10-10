mod article;
mod error;
mod extractor;
mod hashing;
mod prisma;
mod user;

pub type AppResult<T> = Result<T, AppError>;
type AppJsonResult<T> = AppResult<Json<T>>;

use axum::{Json, Router, Server};
use error::{AppError, MainError};
use prisma::PrismaClient;
use std::{env, net::SocketAddr, sync::Arc};

use tower_http::{trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tracing::{debug};

#[derive(Clone)]
pub struct AppState {
    client: Arc<PrismaClient>,
    hmac_key: Arc<String>,
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
    dotenv::dotenv().ok();
    
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_tracing_aka_logging=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client = Arc::new(prisma::new_client().await?);
    let hmac_key = Arc::new(env::var("HMAC_KEY")?);

    let state = Arc::new(AppState { client, hmac_key });

    let app = Router::new()
        .merge(user::routes::create_route(&state))
        .merge(article::routes::create_route(&state))
        .layer(TraceLayer::new_for_http());


    
    println!("Server listening on localhost:5000");
    let addr: SocketAddr = "0.0.0.0:5000".parse()?;

    debug!("Server listening on {}", &addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|_| MainError::BindingError)?;

    Ok(())
}
