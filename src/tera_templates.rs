use crate::api_error::ApiError;
pub use crate::md_struct::*;
use lazy_static::lazy_static;
use markdown::to_html;
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("ressources/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        // tera.autoescape_on(vec!["html", ".sql", "jpg", "svg"]);
        // tera.register_filter("do_nothing", do_nothing_filter);
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
    chapters: Vec<Result<Chapter, ApiError>>,
    offset: i32,
    total: i32,
    manga_id: String,
) -> Result<String, ApiError> {
    let mut context = Context::new();

    let mut chap = vec![];
    for ch in chapters {
        chap.push(ch?)
    }
    context.insert("chapter_list", &chap);
    context.insert("manga_id", &manga_id);

    let rendered = TEMPLATES
        .render("manga_chapter.html", &context)
        .expect("Failed to render chapter template");

    Ok(rendered)
}
