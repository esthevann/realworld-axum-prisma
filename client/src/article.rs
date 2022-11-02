use fake::{Faker, Fake};
use reqwest::{Error, Client};
use types::{article::{Article, NewArticle}, user::User};

use crate::user::create_user;

pub async fn create_article() -> Result<(Article, User), Error> {
    let user = create_user().await?;

    let client = Client::new();
    let article: NewArticle = Faker.fake();

    let req = client
        .post("http://0.0.0.0:5000/api/articles")
        .bearer_auth(&user.user.token)
        .json(&article)
        .send()
        .await?;

    let res: Article = req.json().await?;
    println!("{res:?}");
    Ok((res, user))

}