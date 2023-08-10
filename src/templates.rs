use crate::md_struct::*;
use maud::*;
use std::fs::read;

enum PageOptions {
    Homapage,
    ChapterInfo,
    ReadChapter,
    ErrorPage,
    // DevPage
}

pub fn render_homepage() {
    todo!()
}

pub fn render_manga_info_page() {
    todo!()
}

pub fn render_chapter() {
    todo!()
}

pub fn throw_error() {
    todo!()
}
pub fn render_search_page(search_results: Vec<MangaSearch>) -> String {
    let template = html!(
            (DOCTYPE)
            body {
            h1 {"search results"}
            div.search_list{
                @for i in search_results{
                    div.manga_restult{
                        a href = (format!("/manga/{}",i.manga_name)){
                        img src = { (i.thumbnail)};
                        {(i.manga_name)}
                        }
                    }
            }
            }
        }
    );
    // println!("{}", template.clone().into_string());
    template.into_string()
}
