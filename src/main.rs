use router::Router;
use serde;
use serde_json;
use tera::Tera;
mod md_offline;
mod md_online;
mod templates;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}u suck!", &name)
}
#[get("/manga/{name}")]
async fn get_manga_info(name: web::Path<String>) -> impl Responder {
    format!("Hello {}u suck!", &name)
}
#[get("/manga/{name}/{chapter}")]
async fn get_chapter(name: web::Path<String>) -> impl Responder {
    format!("Hello {}u suck!", &name)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at port 8080");
    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}