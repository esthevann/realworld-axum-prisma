use chrono::{DateTime, FixedOffset};
use serde::{Serialize, Deserialize};

use crate::user::Profile;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommentBody {
    pub id: String,
    pub body: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<FixedOffset>,
    pub author: Profile
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    pub comment: CommentBody
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewComment {
    pub comment: NewCommentBody
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewCommentBody {
    pub body: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comments {
    pub comments: Vec<CommentBody>
}