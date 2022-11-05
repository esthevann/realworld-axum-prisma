use prisma_client_rust::operator::or;
use types::{
    article::{Params, Tags},
    comment::{CommentBody},
    user::{Profile, User, UserBody, ProfileBody},
};

use crate::{
    prisma::{
        user::{self, Data as UserData},
        PrismaClient,
    },
    DbErr,
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

article::include!(article_comment_with_author {
    comments: include {
        author: include {
            favorites
            follows
        }
    }
});

impl UserData {
    pub fn into_user(self, token: String) -> User {
        User {
            user: UserBody {
                email: self.email,
                token,
                username: self.username,
                bio: self.bio,
                image: Some(self.image),
            },
        }
    }
    pub fn into_profile(self, following: bool) -> Profile {
        Profile {
            profile: ProfileBody {
                username: self.username,
                bio: self.bio,
                image: Some(self.image),
                following,
            },
        }
    }
}

impl article_comment_with_author::comments::Data {
    pub fn into_comment_body(self, following: bool) -> CommentBody {
        CommentBody {
            id: self.id,
            body: self.body,
            created_at: self.created_at,
            updated_at: self.updated_at,
            author: Profile {
                profile: ProfileBody { 
                    username: self.author.username,
                    bio: self.author.bio,
                    image: Some(self.author.image),
                    following,
                 },
            },
        }
    }
}

pub struct Query;

impl Query {
    pub async fn get_user_by_id(db: &PrismaClient, id: String) -> Result<UserData, DbErr> {
        let user = db
            .user()
            .find_unique(user::id::equals(id))
            .exec()
            .await?
            .ok_or(DbErr::NotFound)?;

        Ok(user)
    }

    pub async fn get_user_by_email(db: &PrismaClient, email: String) -> Result<UserData, DbErr> {
        let user = db
            .user()
            .find_unique(user::email::equals(email))
            .exec()
            .await?
            .ok_or(DbErr::NotFound)?;

        Ok(user)
    }

    pub async fn get_user_by_username(
        db: &PrismaClient,
        username: String,
    ) -> Result<UserData, DbErr> {
        let user = db
            .user()
            .find_unique(user::username::equals(username))
            .exec()
            .await?
            .ok_or(DbErr::NotFound)?;

        Ok(user)
    }

    pub async fn get_user_favs_and_follows(
        db: &PrismaClient,
        id: String,
    ) -> Result<user_favs_and_follows::Data, DbErr> {
        let user = db
            .user()
            .find_unique(user::id::equals(id))
            .select(user_favs_and_follows::select())
            .exec()
            .await?
            .ok_or(DbErr::NotFound)?;

        Ok(user)
    }

    pub async fn get_user_follows_by_id(
        db: &PrismaClient,
        id: String,
    ) -> Result<Vec<String>, DbErr> {
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
            .ok_or(DbErr::NotFound)?;

        Ok(user.follows.into_iter().map(|x| x.id).collect())
    }

    pub async fn get_article_by_slug(
        db: &PrismaClient,
        slug: String,
    ) -> Result<article_with_user::Data, DbErr> {
        let article = db
            .article()
            .find_unique(article::slug::equals(slug))
            .include(article_with_user::include())
            .exec()
            .await?
            .ok_or(DbErr::NotFound)?;

        Ok(article)
    }

    pub async fn get_articles(
        db: &PrismaClient,
        params: Params,
    ) -> Result<Vec<article_with_user::Data>, DbErr> {
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
    ) -> Result<Vec<article_with_user::Data>, DbErr> {
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

    pub async fn get_tags(db: &PrismaClient) -> Result<Tags, DbErr> {
        let articles = db
            .article()
            .find_many(vec![])
            .select(article::select!({ tag_list }))
            .exec()
            .await?;

        let tags = articles.into_iter().flat_map(|x| x.tag_list).collect();

        Ok(Tags { tags })
    }

    pub async fn get_comments_from_article(
        db: &PrismaClient,
        slug: String,
    ) -> Result<Vec<article_comment_with_author::comments::Data>, DbErr> {
        let comments = db
            .article()
            .find_unique(article::slug::equals(slug))
            .include(article_comment_with_author::include())
            .exec()
            .await?
            .ok_or(DbErr::NotFound)?;

        Ok(comments.comments)
    }
}
