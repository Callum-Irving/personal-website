use crate::db;
use crate::error::MyError;
use crate::models::*;

use actix_web::web::{Json, Path};
use actix_web::{get, post, web, HttpResponse};
use deadpool_postgres::Pool;
use log::debug;

#[get("/comments/{post_id}")]
pub async fn get_comments(post_id: Path<u32>) -> Json<CommentList> {
    debug!("Get comments for: {}", post_id);

    // TODO: Get comments
    let comments = CommentList::new();

    Json(comments)
}

#[post("/createcomment")]
pub async fn create_comment(
    comment: Json<CommentPost>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    debug!(
        "Comment from: {}. On post: {}. Message: {}",
        comment.user, comment.post_id, comment.content
    );

    // TODO: Add profanity filter
    // TODO: Add spam protection

    let comment = comment.into_inner();
    let client = db_pool.get().await.map_err(MyError::PoolError)?;
    let new_comment = db::add_comment(&client, comment).await?;

    Ok(HttpResponse::Ok().json(new_comment))
}
