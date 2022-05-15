//! Backend for my website.
//!
//! This is the backend for the blog comments API for my website at
//! https://callumirving.vercel.app/
//!
//! TODO:
//! - Create UUID system for posts
//! - Use Heroku Postgres to store comments

mod db;
mod error;
mod handlers;
mod models;

use crate::handlers::*;

use ::config::ConfigError;
use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct Config {
    pub pg: deadpool_postgres::Config,
}

impl Config {
    fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::Environment::new())?;
        cfg.try_into()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read environment variables from a .env file for development
    dotenv().ok();

    // Set up logging so we can see what's going on in Heroku console
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Get host and port from environment variables
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_owned());

    // Setup database connection
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify(SslVerifyMode::NONE);
    let connector = MakeTlsConnector::new(builder.build());
    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, connector).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(get_comments)
            .service(create_comment)
            .wrap(Logger::default())
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
