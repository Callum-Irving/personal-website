use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::error::MyError;

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
    pub date: chrono::NaiveDate,
}

impl TryFrom<Row> for Comment {
    type Error = MyError;

    fn try_from(value: Row) -> Result<Self, Self::Error> {
        Ok(Comment {
            user: value.try_get(0).map_err(|_| MyError::DBError)?,
            content: value.try_get(1).map_err(|_| MyError::DBError)?,
            date: value.try_get(2).map_err(|_| MyError::DBError)?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct CommentPost {
    pub post_id: i32,
    pub user: String,
    pub content: String,
}
