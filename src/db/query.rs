use axum::Json;
use prisma_client_rust::operator::or;
use types::{
    article::{Params, Tags},
    user::{Profile, User},
};

use crate::{
    db::prisma::{
        user::{self, Data as UserData},
        PrismaClient,
    },
    error::AppError,
    extractor::AuthUser,
    AppState,
};

use super::{
    mutation::article_with_user,
    prisma::article::{self, WhereParam},
};

user::select!(user_favs_and_follows {
    favorites: select {
        id
    }
    follows: select {
        id
    }
});

impl UserData {
    pub fn into_json(self, state: &AppState) -> Json<User> {
        Json(User {
            email: self.email,
            token: AuthUser { user_id: self.id }.to_jwt(state),
            username: self.username,
            bio: self.bio,
            image: Some(self.image),
        })
    }

    pub fn into_json_profile(self, following: bool) -> Json<Profile> {
        Json(Profile {
            username: self.username,
            bio: self.bio,
            image: Some(self.image),
            following,
        })
    }
}

pub struct Query;

impl Query {
    pub async fn get_user_by_id(db: &PrismaClient, id: String) -> Result<UserData, AppError> {
        let user = db
            .user()
            .find_unique(user::id::equals(id))
            .exec()
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(user)
    }

    pub async fn get_user_by_email(db: &PrismaClient, email: String) -> Result<UserData, AppError> {
        let user = db
            .user()
            .find_unique(user::email::equals(email))
            .exec()
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(user)
    }

    pub async fn get_user_by_username(
        db: &PrismaClient,
        username: String,
    ) -> Result<UserData, AppError> {
        let user = db
            .user()
            .find_unique(user::username::equals(username))
            .exec()
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(user)
    }

    pub async fn get_user_favs_and_follows(
        db: &PrismaClient,
        id: String,
    ) -> Result<user_favs_and_follows::Data, AppError> {
        let user = db
            .user()
            .find_unique(user::id::equals(id))
            .select(user_favs_and_follows::select())
            .exec()
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(user)
    }

    pub async fn get_user_follows_by_id(
        db: &PrismaClient,
        id: String,
    ) -> Result<Vec<String>, AppError> {
        let user = db
            .user()
            .find_unique(user::id::equals(id))
            .select(user::select!({
                follows: select {
                    id
                }
            }))
            .exec()
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(user.follows.into_iter().map(|x| x.id).collect())
    }

    pub async fn get_article_by_slug(
        db: &PrismaClient,
        slug: String,
    ) -> Result<article_with_user::Data, AppError> {
        let article = db
            .article()
            .find_unique(article::slug::equals(slug))
            .include(article_with_user::include())
            .exec()
            .await?
            .ok_or(AppError::NotFound)?;

        Ok(article)
    }

    pub async fn get_articles(
        db: &PrismaClient,
        params: Params,
    ) -> Result<Vec<article_with_user::Data>, AppError> {
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

        let articles = db
            .article()
            .find_many(vec_of_params)
            .order_by(article::created_at::order(
                prisma_client_rust::Direction::Asc,
            ))
            .skip(params.offset.unwrap_or(0))
            .take(params.limit.unwrap_or(20))
            .include(article_with_user::include())
            .exec()
            .await?;

        Ok(articles)
    }

    pub async fn get_followed_articles(
        db: &PrismaClient,
        user_id: String,
        query_params: Params,
    ) -> Result<Vec<article_with_user::Data>, AppError> {
        let user = Query::get_user_follows_by_id(db, user_id).await?;
        let params: Vec<WhereParam> = user
            .into_iter()
            .map(|x| article::user::is(vec![user::id::equals(x)]))
            .collect();

        let articles = db
            .article()
            .find_many(vec![or(params)])
            .order_by(article::created_at::order(
                prisma_client_rust::Direction::Asc,
            ))
            .skip(query_params.offset.unwrap_or(0))
            .take(query_params.limit.unwrap_or(20))
            .include(article_with_user::include())
            .exec()
            .await?;

        Ok(articles)
    }

    pub async fn get_tags(db: &PrismaClient) -> Result<Tags, AppError> {
        let articles = db
            .article()
            .find_many(vec![])
            .select(article::select!({ tag_list }))
            .exec()
            .await?;

        let tags = articles.into_iter().flat_map(|x| x.tag_list).collect();

        Ok(Tags { tags })
    }
}
