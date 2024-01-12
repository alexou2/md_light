mod api_error;
mod consts;
mod downloader;
mod flags;
mod installer;
mod manga_templates;
mod md_struct;
mod online_md;
mod tera_templates;
mod utills;

use actix_files::Files;
use actix_web::{
    get, http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use clap::Parser;
use colored::Colorize;
use local_ip_address::local_ip;
use reqwest::Client;

#[get("/")]
async fn index(path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);
    let feed = online_md::get_md_homepage_feed().await;

    // handles the errors by sending the error page
    // let html = match feed {
    //     Ok(e) => manga_templates::render_homepage(e, is_localhost),
    //     Err(v) => manga_templates::render_error_page(v, "/"),
    // };


    let html = match feed {
        Ok(e) => tera_templates::render_homepage(e),
        Err(v) => manga_templates::render_error_page(v, "/"),
    };
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
    let html = match manga_info {
        // Ok(e) => manga_templates::render_manga_info_page(e, is_localhost),
        Ok(e) => tera_templates::render_manga_info(e),

        Err(v) => manga_templates::render_error_page(v.into(), requested_page),
    };
// let html = "123";
// let ddl = downloader::DownloadData{
//     manga_info: manga_info.unwrap(),
//     source:md_struct::Source::MangaDex,
//     downloaded_language:"en",
//     low_quality_images: false,
//     offset:0

// };

//     downloader::download_manga(Some(ddl));


    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// returns the chapter's pages
#[get("/manga/{manga}/{chapter}")]
async fn get_chapter(chapter: web::Path<(String, String)>, path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);
    let manga_id = chapter.0.to_string();
    let chapter_id = chapter.1.to_string();

    let chapter_info = online_md::get_chapter_pages(chapter_id.clone()).await;
    let html = match chapter_info {
        Ok(e) => manga_templates::render_chapter(e, is_localhost, manga_id),
        Err(v) => manga_templates::render_error_page(v.into(), path.path()),
    };
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// searches for a manga
// #[get("/search/{query}")]
#[get("/search")]
async fn search(path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);
    let query = path.query_string().replace("query=", "");
    println!("{:?}", query);
    // let query: web::Path<String> = todo!();
    let manga_results = online_md::search_manga(Some(query.to_string()), None).await;
    let author_results = online_md::search_author(query.to_string()).await;

    let search_result = manga_results.and_then(|a| author_results.map(|b| (a, b)));

    let html = match search_result {
        // Ok(e) => manga_templates::render_complete_search(e, is_localhost, query.to_string()),
        Ok(e) => tera_templates::render_complete_search(e, query.to_string()),
        Err(v) => manga_templates::render_error_page(v, path.path()),
    };

    // let html = match manga_results {
    //     Ok(e) => tera_templates::render_complete_search(e, query.to_string()),
    //     Err(v) => manga_templates::render_error_page(v, path.path()),
    // };
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// searches for a manga
#[get("/author/{author_id}")]
async fn get_author(author_id: web::Path<String>, path: HttpRequest) -> HttpResponse {
    let author_data = online_md::get_author_infos(author_id.to_string()).await;
    // handles the errors by sending the error page
    let html = match author_data {
        Ok(e) => manga_templates::render_author_page(e),
        Err(v) => manga_templates::render_error_page(v, path.path()),
    };
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/author/{author_id}/feed")]
async fn get_author_feed(author_id: web::Path<String>, path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);
    // println!("path {}", path.query_string().split(",").collect());
    // write("t.txt", path.query_string());
    let manga_list =
        online_md::search_manga(None, Some([("authorOrArtist", author_id.to_string())])).await;

    // handles the errors by sending the error page
    let html = match manga_list {
        Ok(e) => manga_templates::render_author_manga(e, is_localhost),
        Err(v) => manga_templates::render_error_page(v, path.path()),
    };
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/server")]
async fn get_server_options() -> HttpResponse {
    let html = manga_templates::get_server_options();

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// pings the mangadex server to test connection
#[get("/server/ping")]
async fn ping_md() -> impl Responder {
    match online_md::test_connection().await {
        Ok(status) => {
            return format!(
                r"
        reachable: {}
        server up: {}
        ",
                status.reachable, status.up
            )
        }
        Err(v) => return format!("internal server error: {}", v),
    }
}

// kills the server
#[get("/server/kill")]
async fn kill_server(path: HttpRequest) -> impl Responder {
    let restrict = Args::parse().secure;
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
    // "".to_string()
}

async fn image_proxy(image_url: web::Path<String>) -> Result<HttpResponse> {
    let client = Client::new();
    let image_url = image_url.into_inner();

    let response = client.get(&image_url).send().await;

    match response {
        Ok(resp) => {
            let bytes = resp.bytes().await;
            match bytes {
                Ok(image_byte) => Ok(HttpResponse::Ok()
                    // .content_type(resp.headers().get("content-type").unwrap())
                    .body(image_byte)),
                // returns an empty image in case of an error
                Err(e) => {
                    // utills::log_error(e);
                    Ok(HttpResponse::NotFound().finish())
                }
            }
        }
        Err(_) => {
            // Return an error response or a placeholder image
            Ok(HttpResponse::NotFound().finish())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // the launch options
    let mut args = Args::parse();

    // sets the recommended options if launched with `--recommended`
    if args.recommended {
        args.lan = true;
        args.secure = true;
    }

    // prints the options the server will start with
    println!(
        r"Startup options:
    Lan: {lan},
    Port: {port},
    Recommended settings:{recom},
    Secure: {sec}
    ",
        lan = args.lan,
        port = args.port,
        recom = args.recommended,
        sec = args.secure
    );

    // downloads the resources for the frontend, then exits
    if args.install {
        let installer = installer::install_ressources().await;
        match installer {
            Ok(_) => {
                println!("installation successful, now exiting");
                std::process::exit(0);
            }
            Err(e) => {
                println!("error while installing the files: {}", e);
                std::process::exit(1);
            }
        };
    }

    //sets the server port
    let port = args.port;

    println!("Server running at port {}", &port);
    // creates the server
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/proxy/images/{image_url:.+}", web::get().to(image_proxy))
            .service(index)
            .service(kill_server)
            .service(get_server_options)
            .service(get_chapter)
            .service(get_manga_info)
            .service(search)
            .service(ping_md)
            .service(get_author_feed)
            .service(get_author)
            .service(Files::new("/", "/ressources"))
    });

    // the ip addreses used to access the server
    server = server.bind(("127.0.0.1", port))?;
    if args.lan {
        let lan_addr = local_ip().unwrap();
        server = server.bind((lan_addr, port))?;
    }

    server.run().await
}

/// A web server that uses the mangadex api with a lighweight frontend for potato devices
#[derive(Parser, Debug)]
#[command(author = "_alexou_", version = "0.1.2", about , long_about = None)]
pub struct Args {
    /// Creates all of the necessary files and folders for the frontend
    #[arg(short, long)]
    pub install: bool,

    /// Allows other lan devices to connect to the server (you will need to open the port on your device)
    #[arg(short, long)]
    pub lan: bool,

    /// Uses the lower quality images from mangadex instead of the high quality ones
    #[arg(short, long)]
    pub datasaver: bool,

    /// Restricts functionnalities for non-admin users
    #[arg(short, long)]
    pub secure: bool,

    /// Manually set the port for the server
    #[arg(short, long = "PORT", default_value_t = 8080)]
    pub port: u16,

    /// Uses the recommended server options
    #[arg(short, long)]
    pub recommended: bool,
}
