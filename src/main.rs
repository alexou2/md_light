mod api_error;
mod cli_options;
mod downloader;
mod installer;
mod language;
mod manga_templates;
mod md_struct;
mod online_md;
mod query_struct;
mod tera_templates;
mod utills;
mod tests;

use actix_files::Files;
use actix_web::{
    get, http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result,
};
use cli_options::*;
use colored::Colorize;
use lazy_static::lazy_static;
use local_ip_address::local_ip;
use query_struct::*;
use reqwest::Client;
use tera_templates::render_chapter_view;

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

#[get("/")]
async fn index(path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);

    let feed = online_md::get_md_homepage_feed(CONFIG.datasaver).await;

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

    let manga_info = online_md::get_manga_info(manga_id.to_string(), CONFIG.datasaver);

    // handles the errors by sending the error page
    let html = match manga_info.await {
        // Ok(e) => manga_templates::render_manga_info_page(e, is_localhost),
        Ok(e) => tera_templates::render_manga_info(e),

        Err(v) => manga_templates::render_error_page(v, requested_page),
    };

    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// #[derive(serde::Deserialize)]
// struct chapter_query {
//     offset: i32,
//     language: Option<String>,
// }

#[get("/chapters/{id}")]
async fn get_chapters(
    manga_id: web::Path<String>,
    path: HttpRequest,
    infos: web::Query<ChapterQuery>,
) -> HttpResponse {
    let requested_page = path.path();
    let is_localhost = utills::check_localhost(&path);

    let chapters =
        online_md::get_manga_chapters(manga_id.to_string(), infos.language.clone(), infos.offset)
            .await
            .unwrap();

    let html = tera_templates::render_manga_chapters(
        chapters,
        infos.offset,
        manga_id.to_string(),
        is_localhost,
    );

    // handles the errors by sending the error page
    let html = match html {
        // Ok(e) => manga_templates::render_manga_info_page(e, is_localhost),
        Ok(e) => e,
        Err(v) => manga_templates::render_error_page(v, requested_page),
    };

    // let html = "234";
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// returns the chapter's pages
#[get("/manga/{manga}/{chapter}/{chapter_number}")]
async fn get_chapter(chapter: web::Path<(String, String, String)>, path: HttpRequest) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);
    let manga_id = chapter.0.to_string();
    let chapter_id = chapter.1.to_string();
    let chapter_number = chapter.2.to_string().parse::<f32>().unwrap();


    let chapter_info = online_md::get_chapter_pages(chapter_id.clone()).await;
    let infos = online_md::get_prev_and_next_chapters(chapter_id, chapter_number, manga_id.clone(), "en".to_string()).await.unwrap();

    let html = match chapter_info {
        // Ok(e) => manga_templates::render_chapter(e, is_localhost, manga_id),
        Ok(e) => render_chapter_view(e, is_localhost, infos, manga_id),
        Err(v) => manga_templates::render_error_page(v, path.path()),
    };
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(html)
}

// searches for a manga
#[get("/search")]
async fn search(path: HttpRequest, params: web::Query<SearchQuery>) -> HttpResponse {
    let is_localhost = utills::check_localhost(&path);
    let search_query = &params.query;
    println!("search for: {}", search_query);

    let manga_results =
        online_md::search_manga(Some(search_query.to_string()), None, CONFIG.datasaver).await;
    let author_results = online_md::search_author(search_query.to_string()).await;

    let search_result = manga_results.and_then(|a| author_results.map(|b| (a, b)));

    let html = match search_result {
        // Ok(e) => manga_templates::render_complete_search(e, is_localhost, query.to_string()),
        Ok(e) => tera_templates::render_complete_search(e, search_query.to_string()),
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

    let manga_list = online_md::search_manga(
        None,
        Some([("authorOrArtist", author_id.to_string())]),
        CONFIG.datasaver,
    )
    .await;

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
            format!(
                r"
        reachable: {}
        server up: {}
        ",
                status.reachable, status.up
            )
        }
        Err(v) => format!("internal server error: {}", v),
    }
}

// kills the server
#[get("/server/kill")]
async fn kill_server(path: HttpRequest) -> impl Responder {
    let restrict = CONFIG.secure;
    // allows killing the server only if the restrict option is on and the client is the host or if the  restrict option is false
    if (utills::check_localhost(&path)) || !restrict {
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
        format!(
            "You do not have the permission to kill the server\nIP address: {}",
            path.connection_info()
                .peer_addr()
                .expect("unabel to get client IP")
        )
    }
    // "".to_string()
}



// kills the server
#[get("/test")]
async fn test_code() -> impl Responder {
    online_md::tt().await;

    "123".to_string()
}

async fn image_proxy(image_url: web::Path<String>) -> Result<HttpResponse> {
    // let client = Client::new();
    let image_url = image_url.into_inner();

    let response = online_md::CLIENT.get(&image_url).send().await;

    match response {
        Ok(resp) => {
            let bytes = resp.bytes().await;
            match bytes {
                Ok(image_byte) => Ok(HttpResponse::Ok()
                    // .content_type(resp.headers().get("content-type").unwrap())
                    .body(image_byte)),
                // returns an empty image in case of an error
                Err(_) => Ok(HttpResponse::NotFound().finish()),
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
    // creates the config file
    // if CONFIG.command == Some(Commands::Init) {
    //     // creates a mutable version of the startup argments
    //     let mut config_args = CONFIG.to_args().clone();
    //     installer::init(& mut CONFIG.to_args().clone());
    // }
    if let Some(command) = &CONFIG.command {
        match command {
            Commands::Init => installer::init(&mut CONFIG.to_args().clone()),
            Commands::Uninstall => installer::uninstall(),
        }
    }

    println!("{:#?}", CONFIG.to_args());

    // creates the server
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/proxy/images/{image_url:.+}", web::get().to(image_proxy))
            .service(test_code)
            .service(index)
            .service(kill_server)
            .service(get_server_options)
            .service(get_chapter)
            .service(get_manga_info)
            .service(search)
            .service(ping_md)
            .service(get_author_feed)
            .service(get_author)
            .service(get_chapters)
            .service(Files::new("/", "/ressources"))
            
    });

    // the ip addreses used to access the server
    server = server.bind(("127.0.0.1", CONFIG.port))?;
    if CONFIG.lan {
        let lan_addr = local_ip().unwrap();
        server = server.bind((lan_addr, CONFIG.port))?;
        println!("ip address: {}", lan_addr)
    }

    server.run().await
}
