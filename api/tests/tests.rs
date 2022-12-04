use std::sync::Arc;

use axum::{Router, Server};

use db::prisma::PrismaClient;
use fake::{Fake, Faker};
use realworld::{app, AppState};
use reqwest::StatusCode;
use std::net::{SocketAddr, TcpListener};
use types::{user::{NewUserRequest, User}, article::{NewArticle, Article}};

async fn get_app() -> Router {
    dotenvy::dotenv().ok();
    let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let client = Arc::new(
        PrismaClient::_builder()
            .with_url(url)
            .build()
            .await
            .unwrap(),
    );
    let state = AppState {
        client,
        hmac_key: std::env::var("HMAC_KEY")
            .expect("HMAC_KEY must be set")
            .into(),
    };

    app(state.into())
}

#[tokio::test]
async fn basics() {
    let app = get_app().await;
    let listener = TcpListener::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    let client = reqwest::Client::new();
    let user: NewUserRequest = Faker.fake();
    let res = client
        .post(format!("http://{}/api/users", addr))
        .json(&user)
        .send()
        .await
        .expect("Create user request failed");

    assert_eq!(res.status(), StatusCode::OK);
    
    let user_res: User = res.json().await.expect("Failed to serialize to user type");
    assert_eq!(user_res.user.username, user.user.username);
    assert_eq!(user_res.user.bio, "");
    assert_eq!(user_res.user.image, Some("".to_string()));

    let article: NewArticle = Faker.fake();
    let res = client
        .post(format!("http://{}/api/articles", addr))
        .json(&article)
        .header("Authorization", format!("Token {}", user_res.user.token))
        .send()
        .await
        .expect("Create article request failed");

    assert_eq!(res.status(), StatusCode::OK);
    
    let res: Article = res.json().await.expect("Failed to serialize to user type");

    assert_eq!(res.article.author.profile.username, user_res.user.username);
}
