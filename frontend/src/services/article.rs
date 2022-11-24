use super::ApiError;
use gloo_net::http::Request;
use types::article::{MultipleArticles, Article};

pub async fn get_articles(username: &str) -> Result<MultipleArticles, ApiError> {
    let resp = Request::get(&format!("/api/articles?author={}", username))
        .send()
        .await
        .map_err(|_| ApiError::NotFound)?;

    if !resp.ok() {
        if resp.status() == 404 {
            return Err(ApiError::NotFound);
        }
        return Err(ApiError::ServerError);
    }

    resp.json::<MultipleArticles>()
        .await
        .map_err(|_| ApiError::ServerError)
}

pub async fn get_article(slug: &str) -> Result<Article, ApiError> {
    let resp = Request::get(&format!("/api/articles/{}", slug))
        .send()
        .await
        .map_err(|_| ApiError::NotFound)?;

    if !resp.ok() {
        if resp.status() == 404 {
            return Err(ApiError::NotFound);
        } 
        return Err(ApiError::ServerError);
    }

    resp.json::<Article>()
        .await
        .map_err(|_| ApiError::ServerError)
}