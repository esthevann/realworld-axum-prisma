pub mod user;
pub mod article;

#[derive(Clone)]
pub enum ApiError {
    ServerError,
    NotFound
}