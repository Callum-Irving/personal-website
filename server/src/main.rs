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

use actix_web::middleware::Logger;
use actix_web::web;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use std::env;
use url::Url;

fn config_from_url(url: &str) -> Result<deadpool_postgres::Config, &'static str> {
    // Config URL is in format:
    // postgres://USER:PASSWORD@HOST:PORT/DBNAME
    let url = Url::parse(url).map_err(|_| "could not parse db url")?;
    let user = url.username().to_string();
    let password = url.password().ok_or("bad password")?.to_string();
    let host = url.host().ok_or("bad host")?.to_string();
    let port = url.port().ok_or("no port in url")?;
    let dbname = url
        .path_segments()
        .ok_or("cannot be base")?
        .next()
        .unwrap()
        .to_string();

    let mut cfg = deadpool_postgres::Config::new();
    cfg.user = Some(user);
    cfg.password = Some(password);
    cfg.host = Some(host);
    cfg.port = Some(port);
    cfg.dbname = Some(dbname);

    Ok(cfg)
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
    let config = config_from_url(&env::var("DATABASE_URL").unwrap()).unwrap();
    let pool = config.create_pool(None, connector).unwrap();

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
