use chrono::{DateTime, FixedOffset};
use serde::{Serialize, Deserialize};
#[cfg(feature = "fake")]
use fake::{Dummy, Fake};
#[cfg(feature = "fake")]
use fake::faker::lorem::en::Sentence;

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
#[cfg_attr(feature = "fake", derive(Dummy))]
pub struct NewComment {
    pub comment: NewCommentBody
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "fake", derive(Dummy))]
pub struct NewCommentBody {
    #[cfg_attr(feature = "fake", dummy(faker = "Sentence(5..8)"))]
    pub body: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comments {
    pub comments: Vec<CommentBody>
}