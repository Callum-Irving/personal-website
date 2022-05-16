use crate::error::MyError;
use crate::models::*;

use deadpool_postgres::Client;

pub async fn add_comment(client: &Client, comment: CommentPost) -> Result<Comment, MyError> {
    let _stmt = include_str!("../sql/add_comment.sql");
    let stmt = client.prepare(_stmt).await.unwrap();

    // TODO: Check parent exists before inserting
    let res = client
        .query_one(&stmt, &[&comment.post_slug, &comment.user, &comment.content, &comment.parent])
        .await
        .unwrap();

    res.try_into()
}

pub async fn get_comments(client: &Client, post_slug: String) -> Result<CommentList, MyError> {
    let _stmt = include_str!("../sql/get_comments.sql");
    let stmt = client.prepare(_stmt).await.unwrap();

    let res = client
        .query(&stmt, &[&post_slug])
        .await
        .map_err(MyError::Postgres)?;

    let comments: Vec<Comment> = res
        .into_iter()
        .map(|row| row.try_into())
        .collect::<Result<Vec<Comment>, MyError>>()?;

    Ok(CommentList { comments })
}
