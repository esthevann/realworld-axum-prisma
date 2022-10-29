use fake::{Faker, Fake};
use reqwest::{Error, Client};
use types::user::{User, NewUserRequest};


pub async fn create_user() -> Result<User, Error> {
    let client = Client::new();
    let user: NewUserRequest = Faker.fake();
    let req = client.post("http://0.0.0.0:5000/api/users")
        .json(&user)
        .send()
        .await?;

    let res: User = req.json().await?;
    println!("{res:?}");
    Ok(res)
}