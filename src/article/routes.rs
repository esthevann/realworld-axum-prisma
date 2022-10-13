use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};

use crate::{
    error::AppError,
    extractor::MaybeAuthUser,
    prisma::{article, tag, user},
    profiles::types::Profile,
    util::{check_if_favorited, check_if_following},
    AppJsonResult, AppState,
};

use super::types::{Article, Params};

pub fn create_route(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/api/articles", get(handle_list_articles))
        .route("/api/articles/:slug", get(handle_get_article))
}

async fn handle_list_articles(
    MaybeAuthUser(maybe_user): MaybeAuthUser,
    Query(params): Query<Params>,
    State(state): State<AppState>,
) -> AppJsonResult<Vec<Article>> {
    let vec_of_params: Vec<article::WhereParam> = [
        params
            .author
            .map(|x| article::user::is(vec![user::username::equals(x)])),
        params
            .favorited
            .map(|x| article::favorites::some(vec![user::username::equals(x)])),
        params
            .tag
            .map(|x| article::tag_list::some(vec![tag::name::equals(x)])),
    ]
    .into_iter()
    .flatten()
    .collect();

    let articles = state
        .client
        .article()
        .find_many(vec_of_params)
        .skip(params.offset.unwrap_or(0))
        .take(params.limit.unwrap_or(20))
        .include(article::include!({
            user
            tag_list
            favorites
        }))
        .exec()
        .await?;

    let logged_user = if let Some(logged_user) = maybe_user {
        state
            .client
            .user()
            .find_unique(user::id::equals(logged_user.user_id))
            .with(user::favorites::fetch(vec![]))
            .with(user::follows::fetch(vec![]))
            .exec()
            .await?
    } else {
        None
    };

    let articles = articles
        .into_iter()
        .map(|x| Article {
            slug: x.slug,
            title: x.title,
            description: x.description,
            body: x.body,
            tag_list: x.tag_list.into_iter().map(|x| x.name).collect(),
            created_at: x.created_at,
            updated_at: x.updated_at,
            favorited: if let Some(logged_user) = &logged_user {
                check_if_favorited(logged_user, &x.id)
            } else {
                false
            },
            favorites_count: 0,
            author: Profile {
                following: if let Some(logged_user) = &logged_user {
                    check_if_following(logged_user, &x.user)
                } else {
                    false
                },
                username: x.user.username,
                bio: x.user.bio,
                image: Some(x.user.image),
            },
        })
        .collect();

    Ok(Json(articles))
}

async fn handle_get_article(
    MaybeAuthUser(maybe_user): MaybeAuthUser,
    Path(slug): Path<String>,
    State(state): State<AppState>,
) -> AppJsonResult<Article> {
    let article = state
        .client
        .article()
        .find_unique(article::slug::equals(slug))
        .include(article::include!({
            user
            tag_list
            favorites
        }))
        .exec()
        .await?
        .ok_or(AppError::NotFound)?;

    let logged_user = if let Some(logged_user) = maybe_user {
        state
            .client
            .user()
            .find_unique(user::id::equals(logged_user.user_id))
            .with(user::favorites::fetch(vec![]))
            .with(user::follows::fetch(vec![]))
            .exec()
            .await?
    } else {
        None
    };

    Ok(Json(Article {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tag_list: article.tag_list.into_iter().map(|article| article.name).collect(),
            created_at: article.created_at,
            updated_at: article.updated_at,
            favorited: if let Some(logged_user) = &logged_user {
                check_if_favorited(logged_user, &article.id)
            } else {
                false
            },
            favorites_count: 0,
            author: Profile {
                following: if let Some(logged_user) = &logged_user {
                    check_if_following(logged_user, &article.user)
                } else {
                    false
                },
                username: article.user.username,
                bio: article.user.bio,
                image: Some(article.user.image),
            },
        }))
}
