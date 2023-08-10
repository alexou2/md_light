// use router::Router;
// use serde;
// use serde_json;
// use tera::Tera;
mod md_struct;
mod online_md;
mod templates;
mod utills;
use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, http::StatusCode};
use reqwest::header::HeaderName;
use std::{
    os::unix::process,
    time::{Duration, SystemTime},
};
use serde_json::Value;
use serde_json::*;
#[get("/")]
async fn index() -> HttpResponse {
    //    let t = online_md::test_connection().await;
    //    println!("{}", t.unwrap());
    let feed = online_md::get_md_homepage_feed().await.unwrap();
    // match online_md::test_connection().await {
    //     Ok(e) => return "connection established",
    //     Err(v) => return "no connection with the server",
    // }
    let popular = online_md::get_popular_manga().await.unwrap();
    let html = templates::render_homepage(popular);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}u suck!", &name)
}
#[get("/manga/{id}")]
async fn get_manga_info(name: web::Path<String>) -> impl Responder {
    format!("infos about {} not found", &name)
}

#[get("/manga/{manga}/{chapter}")]
async fn get_chapter(path: web::Path<(String, String)>) -> impl Responder {
    let manga_name: String = path.0.to_string();
    let chapter_number: String = path.1.to_string();

    format!("Manga: {}, Chapter: {}", manga_name, chapter_number)
}

#[get("/search/{query}")]
async fn search_for_manga(name: web::Path<String>) -> HttpResponse {
    
    let t = online_md::search_manga(name.to_string()).await.unwrap();
    // format!("search for {}", name)
    // let y = &t[1].manga_name.to_string();
    // y.to_owned()
  let html =   templates::render_search_page(t);
    // "<h1>sdfsdf</h1>".to_string()
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)

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
            .service(search_for_manga)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
