use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CommentList {
    pub comments: Vec<Comment>,
}

impl CommentList {
    pub fn new() -> Self {
        Self { comments: vec![] }
    }
}

#[derive(Serialize)]
pub struct Comment {
    pub user: String,
    pub content: String,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommentPost {
    pub post_id: i32,
    pub user: String,
    pub content: String,
}
