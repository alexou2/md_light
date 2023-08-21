use crate::md_struct::*;
use maud::*;

pub fn render_homepage(popular_manga: Vec<PopularManga>) -> String {
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="/ressources/styles.css";
            body {
            h1 {"HOME"}
            div.search_list{
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
    let template = html!(
        (DOCTYPE)
        link rel="stylesheet" href="/ressources/styles.css";
        body{
            div.manga_info{
        img src = {(manga_info.thumbnail)};
        h1 {(manga_info.manga_name)}
        h3{"authors: "}
            @for author in manga_info.author.unwrap(){
            p{(author.author_name)": "(author.role)}
            };
            {(manga_info.description.unwrap())}
        }
            @for chapter in manga_info.chapters{
                a href = (format!("/manga/{manga_id}/{chapter}", manga_id = manga_info.manga_id, chapter = chapter.chapter_id )){
                    p{(chapter.chapter_name)": "(chapter.language)}
                }
            };
        }
    );
    println!("{}", manga_info.manga_id);
    template.into_string()
}

pub fn render_chapter(chapter_info: ChapterInfo) -> String {
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="/ressources/styles.css";
            body {
            h1 {(chapter_info.chapter_name)}
            div.page_list{
                @for i in chapter_info.pages{
                    img.chapter_page src = (i);
                }
            }
        }
    );
    // println!("{}", template.clone().into_string());
    template.into_string()
}

pub fn render_search_page(search_results: Vec<MangaSearch>) -> String {
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="/ressources/styles.css";
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
