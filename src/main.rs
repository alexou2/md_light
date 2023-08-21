// use router::Router;
// use serde;
// use serde_json;
// use tera::Tera;
mod md_struct;
mod online_md;
mod templates;
mod utills;
use std::net::{IpAddr, Ipv4Addr};

use actix_files::Files;
use actix_web::{
    cookie::time::util, get, http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use clap::Parser;
use local_ip_address::local_ip;
use reqwest::Client;

#[get("/")]
async fn index(path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(path);
    let popular = online_md::get_popular_manga().await.unwrap();
    let html = templates::render_homepage(popular, is_localhost);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/manga/{id}")]
async fn get_manga_info(manga_id: web::Path<String>, path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(path);

    let manga_info = online_md::get_manga_info(manga_id.to_string())
        .await
        .unwrap();
    let html = templates::render_manga_info_page(manga_info, is_localhost);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// returns the chapter's pages
#[get("/manga/{manga}/{chapter}")]
async fn get_chapter(chapter: web::Path<(String, String)>, path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(path);

    let chapter_id: String = chapter.1.to_string();
    let chapter_info = online_md::get_chapter_pages(chapter_id).await;
    let html = templates::render_chapter(chapter_info.unwrap(), is_localhost);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// searches for a manga
#[get("/search/{query}")]
async fn search_for_manga(name: web::Path<String>, path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(path);

    let search_results = online_md::search_manga(name.to_string()).await.unwrap();
    let html = templates::render_search_page(search_results, is_localhost);
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
    ""
}

async fn image_proxy(image_url: web::Path<String>) -> Result<HttpResponse> {
    let client = Client::new();
    let image_url = image_url.into_inner();

    let response = client.get(&image_url).send().await;

    match response {
        Ok(resp) => {
            let bytes = resp.bytes().await.unwrap();
            Ok(HttpResponse::Ok()
                // .content_type(resp.headers().get("content-type").unwrap())
                .body(bytes))
        }
        Err(_) => {
            // Return an error response or a placeholder image
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // manages the cli options
    let args = Args::parse();
    let mut lan_addr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    if args.lan {
        lan_addr = local_ip().unwrap();
        println!("local ip address is: {}", lan_addr);
    }
    println!("{}", lan_addr);

    println!("Server running at port 8080");
    HttpServer::new(|| {
        App::new()
            .route("/proxy/images/{image_url:.+}", web::get().to(image_proxy))
            .service(index)
            .service(kill_server)
            .service(get_chapter)
            .service(get_manga_info)
            .service(search_for_manga)
            .service(ping_md)
            .service(Files::new("/", "/ressources"))
    })
    // the ip addreses used to access the server
    // .bind(("127.0.0.1", 8080))?
    .bind((lan_addr, 8080))?
    .run()
    .await
}
// manages all of the arguments like creating the correct folders

/// A web server that uses the mangadex api with a lighweight frontend for potato devices
#[derive(Parser, Debug)]
#[command(author = "_alexou_", version, about, long_about = None)]
pub struct Args {
    /// Creates all of the necessary files and folders for the program to run
    #[arg(short, long)]
    pub install: bool,

    /// Allows other lan devices to connect to the server
    #[arg(short, long)]
    pub lan: bool,

    /// Uses the lower quality images from mangadex instead of the high quality ones
    #[arg(short, long)]
    pub saver: bool,

    /// Restricts download access for other users on the lan
    #[arg(short, long)]
    pub restrict: bool,
}

use actix_web::Result;
