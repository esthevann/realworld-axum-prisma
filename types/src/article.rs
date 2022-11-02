use chrono::{DateTime, FixedOffset};
use serde::{Serialize, Deserialize};
use fake::{Dummy, Fake};
use fake::faker::lorem::en::{Sentence, Words};

use crate::user::Profile;

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub favorited: bool,
    pub favorites_count: i32,
    pub author: Profile
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub tag: Option<String>,
    pub author: Option<String>,
    pub favorited: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>
}

#[derive(Serialize, Deserialize, Dummy)]
pub struct NewArticle {
    #[dummy(faker = "Sentence(1..3)")]
    pub title: String,
    #[dummy(faker = "Sentence(1..4)")]
    pub description: String,
    #[dummy(faker = "Sentence(5..8)")]
    pub body: String,
    #[dummy(faker = "Words(2..3)")]
    pub tag_list: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, Dummy)]
pub struct UpdateArticle {
    #[dummy(faker = "Sentence(1..3)")]
    pub title: Option<String>,
    #[dummy(faker = "Sentence(1..4)")]
    pub description: Option<String>,
    #[dummy(faker = "Sentence(5..8)")]
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tags {
    pub tags: Vec<String>
}