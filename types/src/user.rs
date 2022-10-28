use fake::{Dummy, Fake};
use fake::faker::internet::en::{Username, Password, FreeEmail};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    pub following: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    display_name: String,
}

#[derive(Serialize, Deserialize, Dummy)]
pub struct NewUserRequest {
    #[dummy(faker = "Username()" )]
    pub username: String,
    #[dummy(faker = "Password(5..12)")]
    pub email: String,
    #[dummy(faker = "FreeEmail()")]
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserResponse {
    pub user: User
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub token: String,
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Default, PartialEq, Serialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub image: Option<String>,
    pub bio: Option<String>,
}