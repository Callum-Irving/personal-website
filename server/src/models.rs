use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::error::MyError;

#[derive(Serialize)]
pub struct CommentList {
    pub comments: Vec<Comment>,
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
            user: value.try_get("username").map_err(|_| MyError::Database)?,
            content: value.try_get("content").map_err(|_| MyError::Database)?,
            date: value.try_get("posted").map_err(|_| MyError::Database)?,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct CommentPost {
    pub post_slug: String,
    pub user: String,
    pub content: String,
}
