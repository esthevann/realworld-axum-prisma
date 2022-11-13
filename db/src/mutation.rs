use prisma_client_rust::QueryError;
use types::{
    article::{Article, ArticleBody, NewArticle, UpdateArticle},
    comment::{Comment, NewComment, CommentBody},
    user::{NewUserRequest, Profile, UpdateUser, ProfileBody},
};

use crate::DbErr;

use super::prisma::{
    article, comment,
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

comment::include!(comment_with_author {
    author: include {
        favorites
        follows
    }
});

pub trait ArticleToJson {
    fn into_article_body(self, following: bool, favorited: bool) -> ArticleBody;
    fn into_article(self, following: bool, favorited: bool) -> Article;
}

pub type ArticleData = article_with_user::Data;
impl ArticleToJson for ArticleData {
    fn into_article(self, following: bool, favorited: bool) -> Article {
        Article {
            article: self.into_article_body(following, favorited)
        }
    }
    fn into_article_body(self, following: bool, favorited: bool) -> ArticleBody {
        ArticleBody {
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
                profile: ProfileBody {
                    following,
                    username: self.user.username,
                    bio: self.user.bio,
                    image: Some(self.user.image),
                }
            },
        }
    }
}

impl comment_with_author::Data {
    pub fn into_comment(self, following: bool) -> Comment {
        Comment {
            comment: CommentBody {
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
            },
        }
    }
}

pub struct Mutation;

impl Mutation {
    pub async fn create_article(
        db: &PrismaClient,
        input: NewArticle,
        author: String,
    ) -> Result<ArticleData, DbErr> {
        let article = db
            .article()
            .create(
                slug::slugify(&input.article.title),
                input.article.title,
                input.article.description,
                input.article.body,
                user::id::equals(author),
                vec![article::tag_list::set(input.article.tag_list)],
            )
            .include(article_with_user::include())
            .exec()
            .await
            .map_err(DbErr::QueryError)?;

        Ok(article)
    }

    pub async fn create_user(
        db: &PrismaClient,
        input: NewUserRequest,
    ) -> Result<user::Data, DbErr> {
        let user = db
            .user()
            .create(
                input.user.username,
                input.user.email,
                input.user.password,
                vec![],
            )
            .exec()
            .await?;

        Ok(user)
    }

    pub async fn update_user(
        db: &PrismaClient,
        id: String,
        update: UpdateUser,
    ) -> Result<user::Data, DbErr> {
        let vec_of_fields: Vec<SetParam> = [
            update.user.email.map(user::email::set),
            update.user.username.map(user::username::set),
            update.user.bio.map(user::bio::set),
            update.user.image.map(user::image::set),
            update.user.password.map(user::password::set),
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
    ) -> Result<article_with_user::Data, DbErr> {
        let vec_of_fields: Vec<article::SetParam> = [
            update
                .article.title
                .as_ref()
                .map(|x| article::slug::set(slug::slugify(x))),
            update.article.title.map(article::title::set),
            update.article.body.map(article::body::set),
            update.article.description.map(article::description::set),
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
            .ok_or(DbErr::NotFound)?
            .user
            .id;

        if article_id != user_id {
            return Err(DbErr::Unauthorized);
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
    ) -> Result<(), DbErr> {
        let article_id = db
            .article()
            .find_unique(article::slug::equals(slug.clone()))
            .select(article::select!({
                user: select {
                    id
                }
            }))
            .exec()
            .await
            .map_err(DbErr::QueryError)?
            .ok_or(DbErr::NotFound)?
            .user
            .id;

        if article_id != user_id {
            return Err(DbErr::Unauthorized);
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
        favorite: bool,
    ) -> Result<article_with_user::Data, DbErr> {
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

    pub async fn create_comment(
        db: &PrismaClient,
        input: NewComment,
        slug: String,
        user_id: String,
    ) -> Result<comment_with_author::Data, DbErr> {
        let comment = db
            .comment()
            .create(
                article::slug::equals(slug),
                input.comment.body,
                user::id::equals(user_id),
                vec![],
            )
            .include(comment_with_author::include())
            .exec()
            .await?;

        Ok(comment)
    }

    pub async fn delete_comment(
        db: &PrismaClient,
        id: String,
        user_id: String,
    ) -> Result<(), DbErr> {
        let comment = db
            .comment()
            .find_unique(comment::id::equals(id))
            .exec()
            .await?;

        if let Some(comment) = comment {
            if comment.user_id != user_id {
                return Err(DbErr::Unauthorized);
            }

            db.comment()
                .delete(comment::id::equals(comment.id))
                .exec()
                .await?;
        }

        Ok(())
    }
}
