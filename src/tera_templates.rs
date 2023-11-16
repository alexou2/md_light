use std::string;

pub use crate::md_struct::*;
use lazy_static::lazy_static;
use serde::ser::SerializeStruct;
use serde_json::value::{to_value, Value};
use tera::{Context, Tera};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("ressources/*") {
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
    context.insert("title", "My Rust App");

    context.insert("manga_result", &search_data.0);
    context.insert("manga_number", &search_data.0.len());
    context.insert("query", &query);



    let rendered = TEMPLATES
        .render("search.html", &context)
        .expect("Failed to render template");

    rendered
}
