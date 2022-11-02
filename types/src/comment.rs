use chrono::{DateTime, FixedOffset};
use serde::{Serialize, Deserialize};

use crate::user::Profile;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub id: String,
    pub body: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub author: Profile
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewComment {
    pub comment: CommentBody
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentBody {
    pub body: String
}