// use router::Router;
// use serde;
// use serde_json;
// use tera::Tera;
mod online_md;
mod struct_constructor;
mod templates;
use actix_web::{get, web, App, HttpServer, Responder};
use reqwest::header::HeaderName;
use std::{
    os::unix::process,
    time::{Duration, SystemTime},
};

#[get("/")]
async fn index() -> impl Responder {
    //    let t = online_md::test_connection().await;
    //    println!("{}", t.unwrap());
    match online_md::test_connection().await {
        Ok(e) => return "connection established",
        Err(v) => return "no connection with the server",
    }
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}u suck!", &name)
}
#[get("/manga/{name}")]
async fn get_manga_info(name: web::Path<String>) -> impl Responder {
    format!("infos about {} not found", &name)
}

#[get("/manga/{manga}/{chapter}")]
async fn get_chapter(path: web::Path<(String, String)>) -> impl Responder {
    let manga_name: String = path.0.to_string();
    let chapter_number: String = path.1.to_string();

    format!("Manga: {}, Chapter: {}", manga_name, chapter_number)
}
#[get("/server/kill")]
async fn kill_server() -> impl Responder {
    println!("The server was killed with exit code 1");
    std::process::exit(1);
    "killed"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at port 8080");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hello)
            .service(kill_server)
            .service(get_chapter)
            .service(get_manga_info)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
