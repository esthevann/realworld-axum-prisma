use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub username: String,
    pub bio: String,
    pub image: Option<String>,
    pub following: bool
}