use chrono::{DateTime, FixedOffset};
use serde::{Serialize, Deserialize};

use crate::user::Profile;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Comment {
    pub id: String,
    pub body: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
    pub author: Profile
}