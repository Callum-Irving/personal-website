use crate::db;
use crate::error::MyError;
use crate::models::*;

use actix_web::web::{Json, Path};
use actix_web::{get, post, web};
use deadpool_postgres::Pool;
use log::debug;

#[get("/comments/{post_id}")]
pub async fn get_comments(
    post_id: Path<i32>,
    db_pool: web::Data<Pool>,
) -> Result<Json<CommentList>, actix_web::Error> {
    debug!("Get comments for: {}", post_id);

    // Get all comments with post_id matching
    let client = db_pool.get().await.map_err(MyError::Pool)?;
    let comments = db::get_comments(&client, post_id.into_inner()).await?;

    Ok(Json(comments))
}

#[post("/createcomment")]
pub async fn create_comment(
    comment: Json<CommentPost>,
    db_pool: web::Data<Pool>,
) -> Result<Json<Comment>, actix_web::Error> {
    debug!(
        "Comment from: {}. On post: {}. Message: {}",
        comment.user, comment.post_id, comment.content
    );

    // TODO: Add profanity filter
    // TODO: Add spam protection

    let comment = comment.into_inner();
    let client = db_pool.get().await.map_err(MyError::Pool)?;
    let new_comment = db::add_comment(&client, comment).await?;

    Ok(Json(new_comment))
}
