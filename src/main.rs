use actix_web::Result;
mod database;
mod md_struct;
mod offline_reader;
mod online_md;
mod templates;
mod utills;
use actix_files::Files;
use actix_web::{
    get, http::StatusCode, web, web::Redirect, App, HttpRequest, HttpResponse, HttpServer,
    Responder,
};
use clap::Parser;
use colored::Colorize;
use local_ip_address::local_ip;
use reqwest::Client;
use serde_json::value::Serializer;
use std::{net::{IpAddr, Ipv4Addr}, error::Error};

#[get("/")]
async fn index(path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);
    let feed = online_md::get_md_homepage_feed().await;

    // handles the errors by sending the error page
    let mut html = String::new();
    match feed {
        Ok(e) => html = templates::render_homepage(e, is_localhost),
        Err(v) => html = templates::render_error_page(v, "/"),
    }
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/manga/{id}")]
async fn get_manga_info(manga_id: web::Path<String>, path: HttpRequest) -> HttpResponse {
    let requested_page = path.path();
    let is_localhost = utills::check_localhost(&path);

    let manga_info = online_md::get_manga_info(manga_id.to_string()).await;

    // handles the errors by sending the error page
    let mut html = String::new();
    match manga_info {
        Ok(e) => html = templates::render_manga_info_page(e, is_localhost),
        Err(v) => html = templates::render_error_page(v, requested_page),
    }
    // let html = templates::render_manga_info_page(manga_info, is_localhost);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// returns the chapter's pages
#[get("/manga/{manga}/{chapter}")]
async fn get_chapter(chapter: web::Path<(String, String)>, path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);

    let chapter_id: String = chapter.1.to_string();
    let chapter_info = online_md::get_chapter_pages(chapter_id.clone()).await;

    println!("{} {}", chapter.0.to_string(), chapter_id);

    // handles the errors by sending the error page
    // let mut html = String::new();
    let html = match chapter_info {
        Ok(e) => templates::render_chapter(e, is_localhost),
        Err(v) => templates::render_error_page(v, path.path()),
    };

    // let html = templates::render_chapter(chapter_info.unwrap(), is_localhost);
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// searches for a manga
#[get("/search/{query}")]
async fn search_for_manga(name: web::Path<String>, path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);

    let search_results = online_md::search_manga(name.to_string()).await;

    // handles the errors by sending the error page
    let mut html = String::new();
    match search_results {
        Ok(e) => html = templates::render_search_page(e, is_localhost),
        Err(v) => html = templates::render_error_page(v, path.path()),
    }

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// searches for a manga
#[get("/author/{author_id}")]
async fn get_author(author_id: web::Path<String>, path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);
    println!("\n\n\n\nn\n");
    let author_data = online_md::get_author_infos(author_id.to_string()).await;
    // handles the errors by sending the error page
    let mut html = String::new();

    match author_data {
        Ok(e) => html = templates::render_author_page(e, is_localhost),
        Err(v) => html = templates::render_error_page(v, path.path()),
    }
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}


#[get("/author/{author_id}/feed")]
async fn get_author_feed(author_id: web::Path<String>, path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);
    println!("\n\n\n\nn\n");
    let author_data = online_md::get_author_manga(author_id.to_string()).await;
    // handles the errors by sending the error page
    let mut html = String::new();

    match author_data {
        Ok(e) => html = templates::render_author_manga(e, is_localhost),
        Err(v) => html = templates::render_error_page(v, path.path()),
    }
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// pings the mangadex server to test connection
#[get("/server/ping")]
async fn ping_md() -> impl Responder {
    match online_md::test_connection().await {
        Ok(e) => return format!("connection established\n{}", e),
        Err(v) => return format!("no connection with the server\n{}", v),
    }
}

// kills the server
#[get("/server/kill")]
async fn kill_server(path: HttpRequest) -> impl Responder {
    let restrict = Args::parse().restrict;
    // allows killing the server only if the restrict option is on and the client is the host or if the  restrict option is false
    if (restrict && utills::check_localhost(&path)) || (!restrict) {
        println!("The server was killed with exit code 1");
        std::process::exit(1);
    } else {
        // prints a message
        println!(
            "Unauthorized access to /server/kill: {}",
            path.connection_info()
                .peer_addr()
                .expect("unable to get client IP")
                .on_red()
        );
        return format!(
            "You do not have the permission to kill the server\nIP address: {}",
            path.connection_info()
                .peer_addr()
                .expect("unabel to get client IP")
        );
    }
    "".to_string()
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

    let port = args.port;
    println!("Server running at port {}", &port);

    HttpServer::new(|| {
        App::new()
            .route("/proxy/images/{image_url:.+}", web::get().to(image_proxy))
            .service(index)
            .service(kill_server)
            .service(get_chapter)
            .service(get_manga_info)
            .service(search_for_manga)
            .service(ping_md)
            .service(get_author)
            .service(Files::new("/", "/ressources"))
    })
    // the ip addreses used to access the server
    // .bind(("127.0.0.1", 8080))?
    .bind((lan_addr, port))?
    .run()
    .await
}



/// A web server that uses the mangadex api with a lighweight frontend for potato devices
#[derive(Parser, Debug)]
#[command(author = "_alexou_", version = "0.1.1", about , long_about = None)]
pub struct Args {
    /// Creates all of the necessary files and folders for the program to run
    #[arg(short, long)]
    pub install: bool,

    /// Allows other lan devices to connect to the server (you will need to open the port on your device)
    #[arg(short, long)]
    pub lan: bool,

    /// Uses the lower quality images from mangadex instead of the high quality ones
    #[arg(short, long)]
    pub saver: bool,

    /// Restricts download access for other users on the lan
    #[arg(short, long)]
    pub restrict: bool,

    /// Uses absolutely no javascript (makes the frontend lighter, but some features WILL be broken)
    #[arg(short, long)]
    pub no_js: bool,

    /// Prints messages about the requested pages and errors
    #[arg(short, long)]
    pub verbose: bool,

    /// Manually set the port for the listener
    #[arg(long = "PORT", default_value_t = 8080)]
    pub port: u16,

    /// use the tor network for viewing chapters online
    #[arg(short, long)]
    pub tor: bool,
}
