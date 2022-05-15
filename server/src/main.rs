//! Backend for my website.
//!
//! This is the backend for the blog comments API for my website at
//! https://callumirving.vercel.app/
//!
//! TODO:
//! - Create UUID system for posts
//! - Use Heroku Postgres to store comments

use actix_web::middleware::Logger;
use actix_web::web::{Json, Path};
use actix_web::{get, App, HttpServer};
use env_logger::Env;
use log::debug;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct CommentList {
    comments: Vec<Comment>,
}

impl CommentList {
    pub fn new() -> Self {
        Self { comments: vec![] }
    }
}

#[derive(Serialize)]
struct Comment {
    user: String,
    content: String,
    date: String,
}

#[get("/comments/{post_id}")]
async fn hello(post_id: Path<u32>) -> Json<CommentList> {
    debug!("Get comments for: {}", post_id);

    // TODO: Get comments
    let comments = CommentList::new();

    Json(comments)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging so we can see what's going on in Heroku console
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Get host and port from environment variables
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_owned());

    HttpServer::new(|| App::new().service(hello).wrap(Logger::default()))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
