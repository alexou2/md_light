use std::error::Error;

use crate::md_struct::*;
use maud::*;

fn get_correct_image(is_localhost: bool, image_thumbnail: String) -> String {
    if !is_localhost {
        return format!("/proxy/images/{}", image_thumbnail);
    } else {
        return image_thumbnail;
    }
}

pub fn render_homepage(feed: MdHomepageFeed, is_localhost: bool) -> String {
    let popular_manga = feed.currently_popular;
    let new_chapters = feed.new_chapter_releases;
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="ressources/styles.css";
            body {
            h1 {"HOME"}
            div.popular{
                @for i in popular_manga{
                    div.manga_restult{
                        a href = (format!("/manga/{}",i.manga_id)){

                            // uses the proxied images if not localhost or links the images directly
                            // @if !is_localhost{
                            //     img src = { (format!("/proxy/images/{}", i.thumbnail))};
                            // }@else{
                            //     img src = (i.thumbnail);
                            // }
                            img src = (get_correct_image(is_localhost, i.thumbnail));

                            {(i.manga_name)}
                        }
                    }
            }
            }
            div.new_chapter{
                @for chapter in new_chapters{
                    div.new_chapter{
                a.chapter href = (format!("/manga/{manga_id}/{chapter_id}", manga_id = chapter.manga_id, chapter_id = chapter.chapter_id)){
                    {(chapter.chapter_name) ":" (chapter.language)}
                };
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
                // @if !is_localhost{
                //     img src = { (format!("/proxy/images/{}", manga_info.thumbnail))};
                // }@else{
                //     img src = (manga_info.thumbnail);
                // }
                img src = (get_correct_image(is_localhost, manga_info.thumbnail));
        h1 {(manga_info.manga_name)}
        h3{"authors: "}
            @for author in manga_info.author{
                a href = {"/author/"(author.author_id)}{
                    p{(author.author_name)": "(author.role)}}
            };
            {(manga_info.description)}
        }
            @for chapter in manga_info.chapters{
                a.chapter href = (format!("/manga/{manga_id}/{chapter}", manga_id = manga_info.manga_id, chapter = chapter.chapter_id )){
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
                    // @if !is_localhost{
                    //     img.chapter_page src = { (format!("/proxy/images/{}", i))};
                    // }@else{
                    //     img.chapter_page src = (i);
                    // }
                    img.chapter_page src = (get_correct_image(is_localhost, i));
                }
            }
        }
    );
    template.into_string()
}

pub fn render_search_page(search_results: Vec<ShortMangaInfo>, is_localhost: bool) -> String {
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
                            // @if !is_localhost{
                            //     img src = { (format!("/proxy/images/{}", i.thumbnail))};
                            // }@else{
                            //     img src = (i.thumbnail);
                            // }
                            img src = (get_correct_image(is_localhost, i.thumbnail));
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

// renders the error page and shows the user the
pub fn render_error_page(error_code: Box<dyn Error>, requested_page: &str) -> String {
    // let probable_error_cause = todo!();
    println!("{}", error_code);
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="/ressources/styles.css";
            body{
                div.error_log{
                h1.error_message{"Oops! looks like there is an error"}
                h3.error_message{(error_code)}
                }
                div.retry_buttons{
                    a.go_home href = {"/"}{"Go back to homepage "};
    div {
                    a.retry href = (requested_page){"retry"};
    }

                }
            }
        );
    template.into_string()
}
pub fn render_author_page(author: AuthorInfo) -> String {
    let template = html!(
        (DOCTYPE)
        link rel="stylesheet" href="/ressources/styles.css";
        script src = {"/ressources/index.js"}{}
        body{
            title {(author.name)}
            div.author_name {(author.name)}
            // script {"console.log(`kj`)"}
            div.title_number {(author.titles_id.len())" titles"}
            div #author_manga{
                // @for manga in author.titles{
                //     a.title href = {"/manga/"(manga.id)}{
                //         // img src = (manga.cover_link);
                //         img src = (get_correct_image(is_localhost, manga.cover_link));
                //         (manga.name)
                //     }
                // }
                {(author.titles_id.join(","))}
                
                // button onclick = {"getAuhorChapters(['"(author.titles_id.join("', '"))"'])"}{}
                }
                script {"getAuhorChapters('"(author.id)"')"};
                // script {"getAuhorChapters(['"(author.titles_id.join("', '"))"'])"};

        }
    );
    template.into_string()
}
pub fn render_author_manga(titles:Vec<ShortMangaInfo>, is_localhost: bool)->String{
    let template = html!(
        @for manga in titles{
            a.title href = {"/manga/"(manga.manga_name)}{
                // img src = (manga.cover_link);
                img src = (get_correct_image(is_localhost, manga.thumbnail));
                (manga.manga_name)
            }
        }
    );
    template.into_string()
}