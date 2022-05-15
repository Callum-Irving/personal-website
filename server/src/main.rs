use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_owned());
    let port = env::var("PORT").unwrap_or("3000".to_owned());

    HttpServer::new(|| App::new().service(hello))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
