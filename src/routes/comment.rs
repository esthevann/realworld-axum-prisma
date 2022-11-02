use axum::{
    extract::{Path, State},
    routing::post,
    Json, Router,
};
use types::comment::{Comment, NewComment};

use crate::{
    db::{mutation::Mutation, query::Query},
    extractor::{AuthUser, MaybeAuthUser},
    util::check_if_following,
    AppJsonResult, AppState,
};

use rayon::prelude::*;

pub fn create_routes(router: Router<AppState>) -> Router<AppState> {
    router.route(
        "/api/articles/:slug/comments",
        post(handle_create_comment).get(handle_comments_from_article),
    )
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

    Ok(comment.into_json(is_following))
}

async fn handle_comments_from_article(
    MaybeAuthUser(maybe_user): MaybeAuthUser,
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> AppJsonResult<Vec<Comment>> {
    let comments = Query::get_comments_from_article(&state.client, slug).await?;

    let logged_user = if let Some(logged_user) = maybe_user {
        Some(Query::get_user_favs_and_follows(&state.client, logged_user.user_id).await?)
    } else {
        None
    };

    let comments = comments
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

            x.into_comment(is_following)
        })
        .collect();

    Ok(Json(comments))
}
