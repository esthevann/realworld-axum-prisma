use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};

use crate::{
    error::AppError,
    extractor::AuthUser,
    hashing::{hash_password, verify_password},
    db::{prisma::user::{self, SetParam}, query::Query},
    AppJsonResult, AppState,
};

use types::user::*;

pub fn create_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/api/users", post(handle_create_user))
        .route("/api/users/login", post(handle_login_user))
        .route(
            "/api/user",
            get(handle_get_current_user).put(handle_update_user),
        )
}

async fn handle_create_user(
    State(state): State<AppState>,
    Json(input): Json<NewUserRequest>,
) -> AppJsonResult<NewUserResponse> {
    let hash = hash_password(input.password).await?;
    let user = state
        .client
        .user()
        .create(input.username.clone(), input.email.clone(), hash, vec![])
        .select(user::select!({ id }))
        .exec()
        .await?;

    Ok(Json(NewUserResponse {
        user: User {
            email: input.email,
            token: AuthUser { user_id: user.id }.to_jwt(&state),
            username: input.username,
            bio: "".to_string(),
            image: None,
        },
    }))
}

async fn handle_login_user(
    State(state): State<AppState>,
    Json(input): Json<LoginUser>,
) -> AppJsonResult<User> {
    let user = state
        .client
        .user()
        .find_unique(user::email::equals(input.email))
        .exec()
        .await?
        .ok_or(AppError::NotFound)?;

    verify_password(input.password, user.password).await?;

    Ok(Json(User {
        email: user.email,
        token: AuthUser { user_id: user.id }.to_jwt(&state),
        username: user.username,
        bio: user.bio,
        image: Some(user.image),
    }))
}

async fn handle_get_current_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> AppJsonResult<User> {
    let user = Query::get_user_by_id(&state.client, auth_user.user_id).await?;

    Ok(Json(User {
        email: user.email,
        token: AuthUser { user_id: user.id }.to_jwt(&state),
        username: user.username,
        bio: user.bio,
        image: Some(user.image),
    }))
}

async fn handle_update_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(input): Json<UpdateUser>,
) -> AppJsonResult<User> {
    if input == UpdateUser::default() {
        return handle_get_current_user(auth_user, State(state)).await;
    }

    let hash: Option<SetParam> = if let Some(pssw) = input.password {
        Some(hash_password(pssw).await?)
    } else {
        None
    }
    .map(user::password::set);

    let vec_of_fields: Vec<SetParam> = [
        input.email.map(user::email::set),
        input.username.map(user::username::set),
        input.bio.map(user::bio::set),
        input.image.map(user::image::set),
        hash,
    ]
    .into_iter()
    .flatten()
    .collect();

    let user = state
        .client
        .user()
        .update(user::id::equals(auth_user.user_id), vec_of_fields)
        .exec()
        .await?;

    Ok(Json(User {
        email: user.email,
        token: AuthUser { user_id: user.id }.to_jwt(&state),
        username: user.username,
        bio: user.bio,
        image: Some(user.image),
    }))
}
