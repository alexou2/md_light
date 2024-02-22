use crate::api_error::ApiError;
pub use crate::md_struct::*;
use crate::online_md;
use lazy_static::lazy_static;
use markdown::to_html;
use serde_json::value::{to_value, Value};
use std::collections::HashMap;
use tera::try_get_value;
use tera::{Context, Function, Tera};

// fn custom_function(text: &HashMap<String, Value>) -> Result<String, tera::Error> {
//     // Your custom logic here
//     Ok(format!("Custom function called with input: {:?}", text))
// }

/// uses the proxied url for images
pub fn proxy_url(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let url = try_get_value!("proxy_url", "value", String, value);
    // let localhost = try_get_value!("proxy_url", "is_localhost", bool, value);
    // println!("{}, {}", url, localhost);
    Ok(to_value(url).unwrap())
}
pub fn get_random(_: &HashMap<String, Value>) -> tera::Result<Value> {
    Ok("12".into())
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.register_function("url_for", get_random);
        // tera.autoescape_on(vec!["html", ".sql", "jpg", "svg"]);
        tera.register_filter("proxy_image", proxy_url);

        tera
    };
}

pub fn render_complete_search(
    search_data: (Vec<ShortMangaInfo>, Vec<AuthorInfo>),
    query: String,
    is_localhost: bool,
    embeded_images: bool,
) -> String {
    let mut context = Context::new();
    context.insert("title", "Search | MD_light");

    context.insert("manga_result", &search_data.0);
    context.insert("manga_number", &search_data.0.len());
    context.insert("query", &query);

    context.insert("author_number", &search_data.1.len());

    context.insert("author_list", &search_data.1);

    // data for the images
    let mut proxy_url = "";
    if is_localhost {
        proxy_url = "/proxy/images/"
    }
    context.insert("proxy_url", proxy_url);

    TEMPLATES
        .render("search.html", &context)
        .expect("Failed to render template")
}

pub fn render_homepage(feed: MdHomepageFeed, is_localhost: bool, embeded_images: bool) -> String {
    let mut context = Context::new();

    context.insert("popular_manga", &feed.currently_popular);
    context.insert("new_chapters", &feed.new_chapter_releases);

    // data for the images
    let mut proxy_url = "";
    if is_localhost {
        proxy_url = "/proxy/images/"
    }
    context.insert("proxy_url", proxy_url);

    TEMPLATES
        .render("home.html", &context)
        .expect("Failed to render template")
}

/// renders the manga without the chapters
pub fn render_manga_info(manga: MangaInfo, is_localhost: bool, embeded_images: bool) -> String {
    let mut context = Context::new();

    context.insert("manga_name", &manga.manga_name);

    context.insert("cover", &manga.cover);
    let html = to_html(&manga.description);
    context.insert("description", &html);
    context.insert("authors", &manga.author);
    context.insert("manga_id", &manga.manga_id);

    // data for the images
    let mut proxy_url = "";
    if is_localhost {
        proxy_url = "/proxy/images/"
    }
    context.insert("proxy_url", proxy_url);

    TEMPLATES
        .render("manga_info.html", &context)
        .expect("Failed to render manga info template")
}
/// renders the chapter list of a manga
pub fn render_manga_chapters(
    chapters: MangaChapters,
    offset: i32,
    manga_id: String,
    is_localhost: bool,
    embeded_images: bool,
) -> Result<String, ApiError> {
    let mut context = Context::new();

    let mut chap = vec![];
    for ch in chapters.chapters {
        chap.push(ch?)
    }
    context.insert("chapter_list", &chap);
    context.insert("manga_id", &manga_id);
    context.insert("current", &round_idx(offset));
    context.insert("total", &round_idx(chapters.total));
    context.insert("is_localhost", &is_localhost);

    // data for the images
    let mut proxy_url = "";
    if is_localhost {
        proxy_url = "/proxy/images/"
    }
    context.insert("proxy_url", proxy_url);

    let rendered = TEMPLATES
        .render("manga_chapter.html", &context)
        .expect("Failed to render chapter template");

    Ok(rendered)
}

/// renders the page to read the chapters
pub async fn render_chapter_view(
    mut pages: Vec<String>,
    is_localhost: bool,
    chapter_infos: CurrentChapter,
    manga_id: String,
    embeded_images: bool,
) -> String {
    let mut context = Context::new();
    // th pages and url
    context.insert("is_localhost", &is_localhost);
    context.insert("chapter", &pages);
    context.insert("chapter_name", &chapter_infos.curr_chapter_name);

    // the contrnt for changing chapters
    context.insert("next_chap", &chapter_infos.next);
    context.insert("has_next", &chapter_infos.next.is_some());
    context.insert("prev_chap", &chapter_infos.prev);
    context.insert("has_prev", &chapter_infos.prev.is_some());

    context.insert("manga_id", &manga_id);

    // data for the images
    let mut proxy_url = "";
    if is_localhost {
        proxy_url = "/proxy/images/"
    }
    context.insert("proxy_url", proxy_url);

    if embeded_images {
        pages =  online_md::get_image_data(pages).await;
    };

    TEMPLATES
        .render("read_chapter.html", &context)
        .expect("Failed to render chapter template")
}

/// renders the author page with its titles
pub fn render_author(author_info: AuthorInfo, is_localhost: bool) -> String {
    let mut context = Context::new();

    context.insert("is_localhost", &is_localhost);
    context.insert("author", &author_info);

    TEMPLATES
        .render("author.html", &context)
        .expect("Failed to render chapter template")
}

/// transforms the offset to an index ex: 501 => 6
fn round_idx(x: i32) -> i32 {
    let x: f32 = x as f32;
    (x / 100.0).ceil() as i32
}
