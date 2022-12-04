use axum::{
    extract::{Path, State},
    routing::{post, delete},
    Json, Router, http::StatusCode,
};
use types::comment::{Comment, NewComment, Comments, CommentBody};
use db::{mutation::Mutation, query::Query};

use crate::{
    extractor::{AuthUser, MaybeAuthUser},
    util::check_if_following,
    AppJsonResult, AppState, error::AppError,
};

use rayon::prelude::*;

pub fn create_routes() -> Router<AppState> {
    Router::new()
    .route(
        "/api/articles/:slug/comments",
        post(handle_create_comment).get(handle_comments_from_article),
    )
    .route("/api/articles/:slug/comments/:id", delete(handle_delete_comment))
}

async fn handle_create_comment(
    AuthUser { user_id }: AuthUser,
    Path(slug): Path<String>,
    State(state): State<AppState>,
    Json(input): Json<NewComment>,
) -> AppJsonResult<Comment> {
    let comment = Mutation::create_comment(&state.client, input, slug, user_id.clone()).await?;

    let is_following = {
        let follows = Query::get_user_follows_by_id(&state.client, user_id).await?;
        check_if_following(&follows, &comment.author.id)
    };

    Ok(Json(comment.into_comment(is_following)))
}

async fn handle_comments_from_article(
    MaybeAuthUser(maybe_user): MaybeAuthUser,
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> AppJsonResult<Comments> {
    let comments = Query::get_comments_from_article(&state.client, slug).await?;

    let logged_user = if let Some(logged_user) = maybe_user {
        Some(Query::get_user_favs_and_follows(&state.client, logged_user.user_id).await?)
    } else {
        None
    };

    let comments: Vec<CommentBody> = comments
        .into_par_iter()
        .map(|x| {
            let is_following = if let Some(logged_user) = &logged_user {
                let follows = logged_user
                    .follows
                    .par_iter()
                    .map(|x| x.id.as_str())
                    .collect::<Vec<&str>>();
                check_if_following(&follows, &x.user_id)
            } else {
                false
            };

            x.into_comment_body(is_following)
        })
        .collect();

    Ok(Json(Comments { comments }))
}

async fn handle_delete_comment(
    AuthUser { user_id }: AuthUser,
    Path((_slug, id)): Path<(String, String)>,
    State(state): State<AppState>
) -> Result<StatusCode, AppError> {
    Mutation::delete_comment(&state.client, id, user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}