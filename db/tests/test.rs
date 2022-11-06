use std::sync::Arc;

use db::{mutation::{Mutation, article_with_user}, prisma::PrismaClient};
use fake::{Fake, Faker};
use tokio::sync::OnceCell;
use types::{article::NewArticle, user::NewUserRequest};

static CLIENT: OnceCell<Arc<PrismaClient>> = OnceCell::const_new();

async fn get_client() -> &'static Arc<PrismaClient> {
    CLIENT
        .get_or_init(|| async {
            dotenvy::dotenv().ok();
            let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
            Arc::new(
                PrismaClient::_builder()
                    .with_url(url)
                    .build()
                    .await
                    .unwrap(),
            )
        })
        .await
}

async fn new_user() -> (db::prisma::user::Data, NewUserRequest) {
    let client = get_client().await;
    let user_input: NewUserRequest = Faker.fake();
    let user = Mutation::create_user(client, user_input.clone())
        .await
        .expect("Couldn't create user");
    (user, user_input)
}

async fn new_article() -> (article_with_user::Data, NewArticle) {
    let client = get_client().await;
    let article_input: NewArticle = Faker.fake();
    let (user, _) = new_user().await;
    let article = Mutation::create_article(client, article_input.clone(), user.id)
        .await
        .expect("Couldn't create article");
    (article, article_input)
}

#[tokio::test]
async fn create_user() {
    let (user, user_input) = new_user().await;
    assert_eq!(user_input.user.username, user.username);
    assert_eq!(user_input.user.email, user.email);
}

#[tokio::test]
async fn create_article() {
    let (article, input) = new_article().await;

    assert_eq!(slug::slugify(&input.article.title), article.slug);
    assert_eq!(input.article.tag_list, article.tag_list);
}