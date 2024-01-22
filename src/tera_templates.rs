use crate::api_error::ApiError;
pub use crate::md_struct::*;
use lazy_static::lazy_static;
use markdown::to_html;
use serde_json::value::{to_value, Value};
use serde_json::{from_value, json};
use std::collections::BTreeMap;
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
pub fn get_random(args: &HashMap<String, Value>) -> tera::Result<Value> {
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
) -> String {
    let mut context = Context::new();
    context.insert("title", "Search | MD_light");

    context.insert("manga_result", &search_data.0);
    context.insert("manga_number", &search_data.0.len());
    context.insert("query", &query);

    context.insert("author_number", &search_data.1.len());

    context.insert("author_list", &search_data.1);

    let rendered = TEMPLATES
        .render("search.html", &context)
        .expect("Failed to render template");

    rendered
}

pub fn render_homepage(feed: MdHomepageFeed) -> String {
    let mut context = Context::new();

    context.insert("popular_manga", &feed.currently_popular);
    context.insert("new_chapters", &feed.new_chapter_releases);
    println!("{}", feed.new_chapter_releases[0].language.lang);
    let rendered = TEMPLATES
        .render("home.html", &context)
        // .expect("Failed to render template");
        .unwrap();
    rendered
}

/// renders the manga without the chapters
pub fn render_manga_info(manga: MangaInfo) -> String {
    let mut context = Context::new();

    context.insert("manga_name", &manga.manga_name);

    context.insert("cover", &manga.thumbnail);
    let html = to_html(&manga.description);
    context.insert("description", &html);
    context.insert("authors", &manga.author);
    context.insert("manga_id", &manga.manga_id);

    let rendered = TEMPLATES
        .render("manga_info.html", &context)
        .expect("Failed to render manga info template");

    rendered
}
/// renders the chapter list of a manga
pub fn render_manga_chapters(
    chapters: MangaChapters,
    offset: i32,
    manga_id: String,
    is_localhost: bool,
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

    let rendered = TEMPLATES
        .render("manga_chapter.html", &context)
        .expect("Failed to render chapter template");

    Ok(rendered)
}
/// transforms the offset to an index ex: 501 => 6
fn round_idx(x: i32) -> i32 {
    let x: f32 = x as f32;
    (x / 100.0).ceil() as i32
}
