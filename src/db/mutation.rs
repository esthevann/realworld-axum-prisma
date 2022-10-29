use axum::Json;
use prisma_client_rust::QueryError;
use types::{
    article::{Article, NewArticle},
    user::Profile,
};

use super::prisma::{article, user, PrismaClient};

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
    fn to_json(self, following: bool, favorited: bool) -> Json<Article>;
    fn to_article(self, following: bool, favorited: bool) -> Article;
}

pub type ArticleData = article_with_user::Data;
impl ArticleToJson for ArticleData {
    fn to_article(self, following: bool, favorited: bool) -> Article {
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
    fn to_json(self, following: bool, favorited: bool) -> Json<Article> {
        Json(self.to_article(following, favorited))
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
}
