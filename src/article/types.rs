use chrono::{DateTime, FixedOffset};
use serde::{Serialize, Deserialize};

use crate::profiles::types::Profile;

#[derive(Serialize, Deserialize)]
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