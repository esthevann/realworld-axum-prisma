use super::ApiError;
use gloo_net::http::Request;
use types::article::MultipleArticles;

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
