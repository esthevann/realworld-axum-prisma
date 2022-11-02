use fake::{Dummy, Fake};
use fake::faker::internet::en::{Username, Password, FreeEmail};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    pub following: bool
}

#[derive(Serialize, Deserialize, Dummy, Clone)]
pub struct NewUserRequest {
    #[dummy(faker = "Username()" )]
    pub username: String,
    #[dummy(faker = "Password(5..12)")]
    pub email: String,
    #[dummy(faker = "FreeEmail()")]
    pub password: String,
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