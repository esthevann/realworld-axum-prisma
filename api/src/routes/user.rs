use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};

use db::{mutation::Mutation, query::Query};

use crate::{
    extractor::AuthUser,
    hashing::{hash_password, verify_password},
    AppJsonResult, AppState,
};

use types::user::*;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/api/users", post(handle_create_user))
        .route("/api/users/login", post(handle_login_user))
        .route(
            "/api/user",
            get(handle_get_current_user).put(handle_update_user),
        )
}

async fn handle_create_user(
    State(state): State<AppState>,
    Json(mut input): Json<NewUserRequest>,
) -> AppJsonResult<User> {
    input.user.password = hash_password(input.user.password).await?;
    let user = Mutation::create_user(&state.client, input).await?;

    let user_id = user.id.clone();

    Ok(Json(user.into_user(AuthUser { user_id  }.to_jwt(&state))))
}

async fn handle_login_user(
    State(state): State<AppState>,
    Json(input): Json<LoginUser>,
) -> AppJsonResult<User> {
    let user = Query::get_user_by_email(&state.client, input.user.email).await?;

    verify_password(input.user.password, user.password.clone()).await?;

    let user_id = user.id.clone();

    Ok(Json(user.into_user(AuthUser { user_id  }.to_jwt(&state))))
}

async fn handle_get_current_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> AppJsonResult<User> {
    let user = Query::get_user_by_id(&state.client, auth_user.user_id).await?;

    let user_id = user.id.clone();

    Ok(Json(user.into_user(AuthUser { user_id  }.to_jwt(&state))))
}

async fn handle_update_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(mut input): Json<UpdateUser>,
) -> AppJsonResult<User> {
    if input == UpdateUser::default() {
        return handle_get_current_user(auth_user, State(state)).await;
    }

    if let Some(pssw) = input.user.password {
        input.user.password = Some(hash_password(pssw).await?)
    }

    let user = Mutation::update_user(&state.client, auth_user.user_id, input).await?;
    let user_id = user.id.clone();

    Ok(Json(user.into_user(AuthUser { user_id  }.to_jwt(&state))))
}
