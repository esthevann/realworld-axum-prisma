use axum::Json;
use prisma_client_rust::QueryError;
use types::{
    article::{Article, NewArticle, UpdateArticle},
    user::{NewUserRequest, Profile, UpdateUser},
};

use crate::error::AppError;

use super::prisma::{
    article,
    user::{self, SetParam},
    PrismaClient,
};

article::include!(article_with_user {
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
});

pub trait ArticleToJson {
    fn into_json(self, following: bool, favorited: bool) -> Json<Article>;
    fn into_article(self, following: bool, favorited: bool) -> Article;
}

pub type ArticleData = article_with_user::Data;
impl ArticleToJson for ArticleData {
    fn into_article(self, following: bool, favorited: bool) -> Article {
        Article {
            slug: self.slug,
            title: self.title,
            description: self.description,
            body: self.body,
            tag_list: self.tag_list,
            created_at: self.created_at,
            updated_at: self.updated_at,
            favorited,
            favorites_count: self.favorites.len() as i32,
            author: Profile {
                following,
                username: self.user.username,
                bio: self.user.bio,
                image: Some(self.user.image),
            },
        }
    }
    fn into_json(self, following: bool, favorited: bool) -> Json<Article> {
        Json(self.into_article(following, favorited))
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn create_article(
        db: &PrismaClient,
        input: NewArticle,
        author: String,
    ) -> Result<ArticleData, QueryError> {
        let article = db
            .article()
            .create(
                slug::slugify(&input.title),
                input.title,
                input.description,
                input.body,
                user::id::equals(author),
                vec![article::tag_list::set(input.tag_list)],
            )
            .include(article_with_user::include())
            .exec()
            .await?;

        Ok(article)
    }

    pub async fn create_user(
        db: &PrismaClient,
        input: NewUserRequest,
    ) -> Result<user::Data, QueryError> {
        let user = db
            .user()
            .create(input.username, input.email, input.password, vec![])
            .exec()
            .await?;

        Ok(user)
    }

    pub async fn update_user(
        db: &PrismaClient,
        id: String,
        update: UpdateUser,
    ) -> Result<user::Data, QueryError> {
        let vec_of_fields: Vec<SetParam> = [
            update.email.map(user::email::set),
            update.username.map(user::username::set),
            update.bio.map(user::bio::set),
            update.image.map(user::image::set),
            update.password.map(user::password::set),
        ]
        .into_iter()
        .flatten()
        .collect();

        let user = db
            .user()
            .update(user::id::equals(id), vec_of_fields)
            .exec()
            .await?;

        Ok(user)
    }

    pub async fn follow_unfollow_user(
        db: &PrismaClient,
        user1_id: String,
        user2_id: String,
        follow: bool,
    ) -> Result<Vec<String>, QueryError> {
        let action = |id: String| {
            if follow {
                user::follows::connect(vec![user::id::equals(id)])
            } else {
                user::follows::disconnect(vec![user::id::equals(id)])
            }
        };

        let user = db
            .user()
            .update(user::id::equals(user1_id), vec![action(user2_id)])
            .select(user::select!({ follows: select { id } }))
            .exec()
            .await?;

        Ok(user.follows.into_iter().map(|x| x.id).collect())
    }

    pub async fn update_article(
        db: &PrismaClient,
        update: UpdateArticle,
        slug: String,
        user_id: String,
    ) -> Result<article_with_user::Data, AppError> {
        let vec_of_fields: Vec<article::SetParam> = [
            update
                .title
                .as_ref()
                .map(|x| article::slug::set(slug::slugify(x))),
            update.title.map(article::title::set),
            update.body.map(article::body::set),
            update.description.map(article::description::set),
        ]
        .into_iter()
        .flatten()
        .collect();

        let article_id = db
            .article()
            .find_unique(article::slug::equals(slug.clone()))
            .select(article::select!({
                user: select {
                    id
                }
            }))
            .exec()
            .await?
            .ok_or(AppError::NotFound)?
            .user
            .id;

        if article_id != user_id {
            return Err(AppError::Unathorized);
        }

        let article = db
            .article()
            .update(article::slug::equals(slug), vec_of_fields)
            .include(article_with_user::include())
            .exec()
            .await?;

        Ok(article)
    }

    pub async fn delete_article(
        db: &PrismaClient,
        slug: String,
        user_id: String,
    ) -> Result<(), AppError> {
        let article_id = db
            .article()
            .find_unique(article::slug::equals(slug.clone()))
            .select(article::select!({
                user: select {
                    id
                }
            }))
            .exec()
            .await?
            .ok_or(AppError::NotFound)?
            .user
            .id;

        if article_id != user_id {
            return Err(AppError::Unathorized);
        }

        db.article()
            .delete(article::slug::equals(slug))
            .exec()
            .await?;

        Ok(())
    }

    pub async fn favorite_unfavorite_article(
        db: &PrismaClient,
        slug: String,
        user_id: String,
        favorite: bool
    ) -> Result<article_with_user::Data, AppError> {
        let action = |id: String| {
            if favorite {
                article::favorites::connect(vec![user::id::equals(id)])
            } else {
                article::favorites::disconnect(vec![user::id::equals(id)])
            }
        };

        let article = db
            .article()
            .update(article::slug::equals(slug), vec![action(user_id)])
            .include(article_with_user::include())
            .exec()
            .await?;

        Ok(article)
    }
}
