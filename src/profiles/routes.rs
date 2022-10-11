use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use tracing::debug;

use crate::{
    error::AppError,
    extractor::{AuthUser, MaybeAuthUser},
    prisma::user,
    AppJsonResult, AppState,
};

use super::types::Profile;

pub fn create_route(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/api/profile/:username", get(handle_get_profile))
        .route("/api/profile/:username/follow", get(handle_follow_user))
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

    let following = if let Some(logged_user) = maybe_user {
        let logged_user = state
            .client
            .user()
            .find_unique(user::id::equals(logged_user.user_id))
            .with(user::follows::fetch(vec![]))
            .exec()
            .await?;

        if let Some(logged_user) = logged_user 
            && let Some(follows) = logged_user.follows 
            {
                follows.iter().any(|x| x.id == user.id )
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
        following,
    }))
}

async fn handle_follow_user(
    logged_user: AuthUser,
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

    let logged_user = state
        .client
        .user()
        .update(
            user::id::equals(logged_user.user_id),
            vec![user::follows::connect(vec![user::id::equals(
                user.id.clone(),
            )])],
        )
        .with(user::follows::fetch(vec![]))
        .exec()
        .await?;

    let following = if let Some(follows) = logged_user.follows {
        follows.iter().any(|x| {
            debug!("{}", &x.username);
            x.id == user.id
        })
    } else {
        false
    };

    Ok(Json(Profile {
        username: user.username,
        bio: user.bio,
        image: Some(user.image),
        following,
    }))
}
