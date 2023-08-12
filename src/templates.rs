use crate::{md_struct::*, templates};
use maud::*;

pub fn render_homepage(popular_manga: Vec<PopularManga>) -> String {
    let template = html!(
            (DOCTYPE)
            body {
            h1 {"search results"}
            div.search_list{
                @let mut ranking = 0;
                @for i in popular_manga{
                    div.manga_restult{
                        a href = (format!("/manga/{}",i.manga_id)){
                        img src = { (i.thumbnail)};
                        {(i.manga_name)}
                        }
                    }
            }
            }
        }
    );
    template.into_string()
}

pub fn render_manga_info_page(manga_info: MangaInfo) -> String {
    let template = html!((DOCTYPE)
        img src = {(manga_info.thumbnail)};
        h1 {(manga_info.manga_name)}
        h3{"authors: "}
            @for author in manga_info.author{
            p{(author.author_name)": "(author.role)}
            };

            @for chapter in manga_info.chapters{
                p{(chapter.chapter_name)": "(chapter.language)"        "(chapter.i)}
            };

    );
    println!("{}", manga_info.manga_id);
    template.into_string()
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
                        a href = (format!("/manga/{}",i.manga_id)){
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
