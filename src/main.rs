// use router::Router;
// use serde;
// use serde_json;
// use tera::Tera;
mod md_struct;
mod online_md;
mod templates;
mod utills;
use actix_files::Files;
use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use clap::Parser;
use local_ip_address::local_ip;

#[get("/")]
async fn index() -> HttpResponse {
    let popular = online_md::get_popular_manga().await.unwrap();
    let html = templates::render_homepage(popular);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/manga/{id}")]
async fn get_manga_info(manga_id: web::Path<String>) -> HttpResponse {
    let manga_info = online_md::get_manga_info(manga_id.to_string())
        .await
        .unwrap();
    let html = templates::render_manga_info_page(manga_info);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/manga/{manga}/{chapter}")]
async fn get_chapter(path: web::Path<(String, String)>) -> HttpResponse {
    let chapter_id: String = path.1.to_string();
    let chapter_info = online_md::get_chapter_pages(chapter_id).await;
    let html = templates::render_chapter(chapter_info.unwrap());
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/search/{query}")]
async fn search_for_manga(name: web::Path<String>) -> HttpResponse {
    let t = online_md::search_manga(name.to_string()).await.unwrap();
    // format!("search for {}", name)
    // let y = &t[1].manga_name.to_string();
    // y.to_owned()
    let html = templates::render_search_page(t);
    // "<h1>sdfsdf</h1>".to_string()
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}
#[get("/server/ping")]
async fn ping_md() -> impl Responder {
    match online_md::test_connection().await {
        Ok(e) => return format!("connection established\n{}", e),
        Err(v) => return format!("no connection with the server\n{}", v),
    }
}

#[get("/server/kill")]
async fn kill_server() -> impl Responder {
    println!("The server was killed with exit code 1");
    std::process::exit(1);
    "killed"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // manages the cli options
    manage_args(Args::parse());
    
    let addr = local_ip().unwrap();
    println!("local ip address is: {}", addr);
    println!("Server running at port 8080");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(kill_server)
            .service(get_chapter)
            .service(get_manga_info)
            .service(search_for_manga)
            .service(ping_md)
            .service(Files::new("/", "/ressources"))
    })
    // the ip addreses used to access the server
    .bind(("127.0.0.1", 8080))?
    .bind((addr.to_string(), 8080))?
    .run()
    .await
}
// manages all of the arguments like creating the correct folders
fn manage_args(args: Args) {}

/// A web server that uses the mangadex api with a lighweight frontend for potato devices
#[derive(Parser, Debug)]
#[command(author = "_alexou_", version, about, long_about = None)]
struct Args {
    /// Creates all of the necessary files and folders for the program to run
    #[arg(short, long)]
    install: bool,

    /// Allows other lan devices to connect to the server
    #[arg(short, long)]
    lan: bool,

    /// Uses the lower quality images from mangadex instead of the high quality ones
    #[arg(short, long)]
    saver: bool,

    /// Restricts download access for other users on the lan
    #[arg(short, long)]
    restrict: bool,
}
