use gloo_net::http::Request;
use types::user::Profile;

use super::ApiError;

pub async fn get_profile(username: &str) -> Result<Profile, ApiError> {
    let resp = Request::get(&format!("/api/profiles/{}", username))
        .send()
        .await
        .map_err(|_| ApiError::NotFound)?;

    if !resp.ok() {
        if resp.status() == 404 {
            return Err(ApiError::NotFound);
        }
        return Err(ApiError::ServerError);
    }
    
    resp.json::<Profile>().await.map_err(|_| ApiError::ServerError)
}
