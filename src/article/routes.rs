use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};

use crate::{
    error::AppError,
    extractor::{AuthUser, MaybeAuthUser},
    prisma::{article, user},
    profiles::types::Profile,
    util::{check_if_favorited, check_if_following},
    AppJsonResult, AppState,
};

use super::types::{Article, NewArticle, Params};

pub fn create_route(router: Router<AppState>) -> Router<AppState> {
    router
        .route(
            "/api/articles",
            get(handle_list_articles).post(handle_create_article),
        )
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
        params.tag.map(|x| article::tag_list::has_some(vec![x])),
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
            tag_list: x.tag_list,
            created_at: x.created_at,
            updated_at: x.updated_at,
            favorited: if let Some(logged_user) = &logged_user {
                let favorites = Some(
                    logged_user
                        .favorites
                        .iter()
                        .flat_map(|x| x)
                        .map(|x| x.id.as_str())
                        .collect::<Vec<&str>>(),
                );
                check_if_favorited(&favorites,&x.id,)
            } else {
                false
            },
            favorites_count: 0,
            author: Profile {
                following: if let Some(logged_user) = &logged_user {
                    let follows = Some(
                        logged_user
                            .follows
                            .iter()
                            .flat_map(|x| x)
                            .map(|x| x.id.as_str())
                            .collect::<Vec<&str>>(),
                    );
                    check_if_following(&follows, &x.id)
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
        tag_list: article.tag_list,
        created_at: article.created_at,
        updated_at: article.updated_at,
        favorited: if let Some(logged_user) = &logged_user {
            let favorites = Some(
                logged_user
                    .favorites
                    .iter()
                    .flat_map(|x| x)
                    .map(|x| x.id.as_str())
                    .collect::<Vec<&str>>(),
            );
            check_if_favorited(
                &favorites,
                &article.id,
            )
        } else {
            false
        },
        favorites_count: 0,
        author: Profile {
            following: if let Some(logged_user) = &logged_user {
                let follows = Some(
                    logged_user
                        .follows
                        .iter()
                        .flat_map(|x| x)
                        .map(|x| x.id.as_str())
                        .collect::<Vec<&str>>(),
                );
                check_if_following(
                    &follows,
                    &article.user.id,
                )
            } else {
                false
            },
            username: article.user.username,
            bio: article.user.bio,
            image: Some(article.user.image),
        },
    }))
}

async fn handle_create_article(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(input): Json<NewArticle>,
) -> AppJsonResult<Article> {
    let article = state
        .client
        .article()
        .create(
            slug::slugify(&input.title),
            input.title,
            input.description,
            input.body,
            user::id::equals(auth_user.user_id),
            vec![article::tag_list::set(input.tag_list)],
        )
        .include(article::include!({
            user: select {
                id
                username
                bio
                image
                favorites: select {
                    id
                }
                follows: select {
                    id
                }
            }
            favorites: select {
                id
            }
        }))
        .exec()
        .await?;

    Ok(Json(Article {
        slug: article.slug,
        title: article.title,
        description: article.description,
        body: article.body,
        tag_list: article.tag_list,
        created_at: article.created_at,
        updated_at: article.updated_at,
        favorited: check_if_favorited(
            &Some(article.user.favorites.iter().map(|x| x.id.as_str()).collect()),
            &article.id,
        ),
        favorites_count: article.favorites.len() as i32,
        author: Profile {
            following: check_if_following(
                &Some(article.user.follows.iter().map(|x| x.id.as_str()).collect()),
                &article.user.id,
            ),
            username: article.user.username,
            bio: article.user.bio,
            image: Some(article.user.image),
        },
    }))
}
