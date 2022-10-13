use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};

use crate::{
    error::AppError,
    extractor::{AuthUser, MaybeAuthUser},
    prisma::user,
    AppJsonResult, AppState, util::check_if_following,
};

use super::types::Profile;

pub fn create_route(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/api/profile/:username", get(handle_get_profile))
        .route("/api/profile/:username/follow", post(handle_follow_user).delete(handle_unfollow_user))
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

    let following = if let Some(logged_user) = maybe_user 
                            && let Some(logged_user) =  state
                                                                .client
                                                                .user()
                                                                .find_unique(user::id::equals(logged_user.user_id))
                                                                .with(user::follows::fetch(vec![]))
                                                                .exec()
                                                                .await?
    {
        check_if_following(&logged_user, &user)
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

    let following = check_if_following(&logged_user, &user);

    Ok(Json(Profile {
        username: user.username,
        bio: user.bio,
        image: Some(user.image),
        following,
    }))
}

async fn handle_unfollow_user(
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
            vec![user::follows::disconnect(vec![user::id::equals(
                user.id.clone(),
            )])],
        )
        .with(user::follows::fetch(vec![]))
        .exec()
        .await?;

    let following = check_if_following(&logged_user, &user);

    Ok(Json(Profile {
        username: user.username,
        bio: user.bio,
        image: Some(user.image),
        following,
    }))
}
