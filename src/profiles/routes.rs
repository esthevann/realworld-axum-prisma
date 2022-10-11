use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

use crate::{error::AppError, extractor::MaybeAuthUser, prisma::user, AppJsonResult, AppState};

use super::types::Profile;

pub fn create_route(router: Router<AppState>) -> Router<AppState> {
    router.route("/api/profile/:username", get(handle_get_profile))
}

async fn handle_get_profile(
    MaybeAuthUser(maybe_user): MaybeAuthUser,
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> AppJsonResult<Profile> {
    let user = state
        .client
        .user()
        .find_unique(user::username::equals(username))
        .exec()
        .await?
        .ok_or(AppError::NotFound)?;

    let following: bool = if let Some(logged_user) = maybe_user {
        if let Some(follows) = user.follows {
            let logged_user = state
                .client
                .user()
                .find_unique(user::id::equals(logged_user.user_id))
                .select(user::select!({ username }))
                .exec()
                .await?;

            match logged_user {
                Some(logged_user) => {
                     follows
                        .iter()
                        .any(|x| x.username == logged_user.username)
                }
                None => false,
            }
        } else {
            false
        }
    } else {
        false
    };

    Ok(Json(Profile {        
        username: user.username,
        bio: user.bio,
        image: Some(user.image),
        following
    }))
}
