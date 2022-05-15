use crate::error::MyError;
use crate::models::*;

use deadpool_postgres::Client;

pub async fn add_comment(client: &Client, comment: CommentPost) -> Result<Comment, MyError> {
    let _stmt = include_str!("../sql/add_comment.sql");
    let stmt = client.prepare(_stmt).await.unwrap();

    let res = client
        .query_one(&stmt, &[&comment.post_id, &comment.user, &comment.content])
        .await
        .unwrap();

    let comment = Comment {
        user: res.try_get(0).map_err(|_| MyError::DBError)?,
        content: res.try_get(1).map_err(|_| MyError::DBError)?,
        date: res.try_get(2).map_err(|_| MyError::DBError)?,
    };

    Ok(comment)
}
