use crate::{db::mutation::ArticleToJson, error::AppError};
use axum::{
    extract::{Path, Query as UrlQuery, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use rayon::prelude::*;

use crate::{
    db::{mutation::Mutation, query::Query},
    extractor::{AuthUser, MaybeAuthUser},
    util::{check_if_favorited, check_if_following},
    AppJsonResult, AppState,
};

use types::article::{Article, NewArticle, Params, UpdateArticle, Tags};

pub fn create_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .route(
            "/api/articles",
            get(handle_list_articles).post(handle_create_article),
        )
        .route(
            "/api/articles/:slug",
            get(handle_get_article)
                .put(handle_update_article)
                .delete(handle_delete_article),
        )
        .route("/api/articles/feed", get(handle_feed_articles))
        .route(
            "/api/articles/:slug/favorite",
            post(handle_favorite_article).delete(handle_unfavorite_article),
        )
        .route("/api/tags", get(handle_get_tags))
}

async fn handle_list_articles(
    MaybeAuthUser(maybe_user): MaybeAuthUser,
    UrlQuery(params): UrlQuery<Params>,
    State(state): State<AppState>,
) -> AppJsonResult<Vec<Article>> {
    let articles = Query::get_articles(&state.client, params).await?;

    let logged_user = if let Some(logged_user) = maybe_user {
        Some(Query::get_user_favs_and_follows(&state.client, logged_user.user_id).await?)
    } else {
        None
    };

    let articles = articles
        .into_par_iter()
        .map(|x| {
            let (is_favorited, is_following) = if let Some(logged_user) = &logged_user {
                let (favorites, follows) = (
                    logged_user
                        .favorites
                        .par_iter()
                        .map(|x| x.id.as_str())
                        .collect::<Vec<&str>>(),
                    logged_user
                        .follows
                        .par_iter()
                        .map(|x| x.id.as_str())
                        .collect::<Vec<&str>>(),
                );

                (
                    check_if_favorited(&favorites, &x.id),
                    check_if_following(&follows, &x.user_id),
                )
            } else {
                (false, false)
            };

            x.into_article(is_following, is_favorited)
        })
        .collect();

    Ok(Json(articles))
}

async fn handle_get_article(
    MaybeAuthUser(maybe_user): MaybeAuthUser,
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> AppJsonResult<Article> {
    let article = Query::get_article_by_slug(&state.client, slug).await?;

    let logged_user = if let Some(logged_user) = maybe_user {
        Some(Query::get_user_favs_and_follows(&state.client, logged_user.user_id).await?)
    } else {
        None
    };

    let is_favorited = if let Some(logged_user) = &logged_user {
        let favorites = logged_user
            .favorites
            .par_iter()
            .map(|x| x.id.as_str())
            .collect::<Vec<&str>>();
        check_if_favorited(&favorites, &article.id)
    } else {
        false
    };

    let is_following = if let Some(logged_user) = &logged_user {
        let follows = logged_user
            .follows
            .par_iter()
            .map(|x| x.id.as_str())
            .collect::<Vec<&str>>();
        check_if_following(&follows, &article.user.id)
    } else {
        false
    };

    Ok(article.into_json(is_following, is_favorited))
}

async fn handle_create_article(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(input): Json<NewArticle>,
) -> AppJsonResult<Article> {
    let article = Mutation::create_article(&state.client, input, auth_user.user_id).await?;

    let is_following = check_if_following(
        &article
            .user
            .follows
            .par_iter()
            .map(|x| x.id.as_str())
            .collect::<Vec<&str>>(),
        &article.user.id,
    );

    Ok(article.into_json(is_following, false))
}

pub async fn handle_feed_articles(
    AuthUser { user_id }: AuthUser,
    UrlQuery(params): UrlQuery<Params>,
    State(state): State<AppState>,
) -> AppJsonResult<Vec<Article>> {
    let articles = Query::get_followed_articles(&state.client, user_id.clone(), params).await?;

    let user = Query::get_user_favs_and_follows(&state.client, user_id).await?;

    let favorites = user
        .favorites
        .par_iter()
        .map(|x| x.id.as_str())
        .collect::<Vec<&str>>();

    let articles = articles
        .into_iter()
        .map(|x| {
            let favorited = check_if_favorited(&favorites, &x.id);
            x.into_article(true, favorited)
        })
        .collect();

    Ok(Json(articles))
}

pub async fn handle_update_article(
    AuthUser { user_id }: AuthUser,
    Path(slug): Path<String>,
    State(state): State<AppState>,
    Json(input): Json<UpdateArticle>,
) -> AppJsonResult<Article> {
    let article = Mutation::update_article(&state.client, input, slug, user_id).await?;

    let is_favorited = check_if_favorited(
        &article
            .user
            .favorites
            .par_iter()
            .map(|x| x.id.as_str())
            .collect::<Vec<&str>>(),
        &article.id,
    );

    let is_following = check_if_following(
        &article
            .user
            .follows
            .par_iter()
            .map(|x| x.id.as_str())
            .collect::<Vec<&str>>(),
        &article.user.id,
    );

    Ok(article.into_json(is_following, is_favorited))
}

pub async fn handle_delete_article(
    AuthUser { user_id }: AuthUser,
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> Result<StatusCode, AppError> {
    Mutation::delete_article(&state.client, slug, user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn handle_favorite_article(
    auth_user: AuthUser,
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> AppJsonResult<Article> {
    Mutation::favorite_unfavorite_article(&state.client, slug.clone(), auth_user.user_id.clone(), true).await?;

    handle_get_article(MaybeAuthUser(Some(auth_user)), Path(slug), State(state)).await
}

pub async fn handle_unfavorite_article(
    auth_user: AuthUser,
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> AppJsonResult<Article> {
    Mutation::favorite_unfavorite_article(&state.client, slug.clone(), auth_user.user_id.clone(), false).await?;

    handle_get_article(MaybeAuthUser(Some(auth_user)), Path(slug), State(state)).await
}

pub async fn handle_get_tags(State(state): State<AppState>) -> AppJsonResult<Tags> {
    let tags = Query::get_tags(&state.client).await?;
    Ok(Json(tags))
}