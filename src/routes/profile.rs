use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};

use crate::{
    extractor::{AuthUser, MaybeAuthUser},
    AppJsonResult, AppState, util::check_if_following,
    db::{query::Query, mutation::Mutation}
};

use types::user::Profile;

pub fn create_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/api/profiles/:username", get(handle_get_profile))
        .route("/api/profiles/:username/follow", post(handle_follow_user).delete(handle_unfollow_user))
}

async fn handle_get_profile(
    MaybeAuthUser(maybe_user): MaybeAuthUser,
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> AppJsonResult<Profile> {
    let user = Query::get_user_by_username(&state.client, username).await?;

    let following = if let Some(logged_user) = maybe_user 
    {
        let follows = Query::get_user_follows_by_id(&state.client, logged_user.user_id).await?;
        check_if_following(&follows, &user.id)
    } else {
        false
    };

    Ok(user.into_json_profile(following))
}

async fn handle_follow_user(
    logged_user: AuthUser,
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> AppJsonResult<Profile> {
    let user = Query::get_user_by_username(&state.client, username).await?;

    let follows = Mutation::follow_unfollow_user(
        &state.client, logged_user.user_id, user.id.clone(), true)
        .await?;

    let following = check_if_following(&follows, &user.id);

    Ok(user.into_json_profile(following))
}

async fn handle_unfollow_user(
    logged_user: AuthUser,
    Path(username): Path<String>,
    State(state): State<AppState>,
) -> AppJsonResult<Profile> {
    let user = Query::get_user_by_username(&state.client, username).await?;

    let follows = Mutation::follow_unfollow_user(
        &state.client, logged_user.user_id, user.id.clone(), false)
        .await?;

    let following = check_if_following(&follows, &user.id);

    Ok(user.into_json_profile(following))
}
