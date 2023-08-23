use crate::md_struct::*;
use maud::*;

pub fn render_homepage(popular_manga: Vec<PopularManga>, is_localhost: bool) -> String {
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="/ressources/styles.css";
            body {
            h1 {"HOME"}
            div.search_list{
                @for i in popular_manga{
                    div.manga_restult{
                        a href = (format!("/manga/{}",i.manga_id)){

                            // uses the proxied images if not localhost or links the images directly
                            @if !is_localhost{
                                img src = { (format!("/proxy/images/{}", i.thumbnail))};
                            }@else{
                                img src = (i.thumbnail);
                            }

                            {(i.manga_name)}
                        }
                    }
            }
            }
        }
    );
    template.into_string()
}

pub fn render_manga_info_page(manga_info: MangaInfo, is_localhost: bool) -> String {
    let template = html!(
        (DOCTYPE)
        link rel="stylesheet" href="/ressources/styles.css";
        body{
            div.manga_info{

                // uses the proxied images if not localhost or links the images directly
                @if !is_localhost{
                    img src = { (format!("/proxy/images/{}", manga_info.thumbnail))};
                }@else{
                    img src = (manga_info.thumbnail);
                }
        h1 {(manga_info.manga_name)}
        h3{"authors: "}
            @for author in manga_info.author{
                p{(author.author_name)": "(author.role)}
            };
            {(manga_info.description)}
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

pub fn render_chapter(chapter_info: ChapterInfo, is_localhost: bool) -> String {
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="/ressources/styles.css";
            body {
            h1 {(chapter_info.chapter_name)}
            div.page_list{
                @for i in chapter_info.pages{

                    // uses the proxied images if not localhost or links the images directly
                    @if !is_localhost{
                        img.chapter_page src = { (format!("/proxy/images/{}", i))};
                    }@else{
                        img.chapter_page src = (i);
                    }
                }
            }
        }
    );
    template.into_string()
}

pub fn render_search_page(search_results: Vec<MangaSearch>, is_localhost: bool) -> String {
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="/ressources/styles.css";
            body {
            h1 {"search results"}
            div.search_list{
                @for i in search_results{
                    div.manga_restult{
                        a href = (format!("/manga/{}",i.manga_id)){
                            
                            // uses the proxied images if not localhost or links the images directly
                            @if !is_localhost{
                                img src = { (format!("/proxy/images/{}", i.thumbnail))};
                            }@else{
                                img src = (i.thumbnail);
                            }
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
