use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
pub struct Post {
    post_id: String,
    user_id: String,
    content: String,
    created_at: i64,
    updated_at: i64,
    likes_count: i64,
    dislikes_count: i64,
    comments_count: i64,
    flags: Vec<String>,
    media: Vec<String>,
    media_type: Option<String>,
    status: Option<String>,
    is_deleted: Option<bool>,
    tags: Option<Vec<String>>,
}
