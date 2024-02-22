use crate::api_error::ApiError;
use crate::language::*;
use crate::md_struct::*;
use maud::*;

fn get_top_bar() -> PreEscaped<String> {
    let top_bar = html!(
        link rel="stylesheet" href="/ressources/styles.css";
        script src = {"/ressources/index.js"}{}
        link rel="icon" href="/ressources/feather.svg" type="image/x-icon";
        div.top_bar{
                a.go_home href = "/"{
                    img.logo src = "/ressources/logo.svg";
                }

                div.search_bar{
                    input #search_box action = "search()" type = "text" title = "search" required;
                    button type="button" onclick = "search()" {"Search"}

                }
        }
    );

    top_bar
}

fn get_desc(desc: String) -> PreEscaped<String> {
    html! {(PreEscaped(&desc))}
}

fn get_return_to_manga(manga_id: String) -> PreEscaped<String> {
    // let manga_id = path.split("/").collect::<Vec<&str>>()[2];
    let home = html!(

        div.bottom_bar{
            img.feather src = "/ressources/feather.svg";
            h1.go_home{a href = (format!("/manga/{}", manga_id)); "back to manga"}

        }
    );
    home
}

fn get_correct_image(is_localhost: bool, image_thumbnail: String) -> String {
    if !is_localhost {
        format!("/proxy/images/{}", image_thumbnail)
    } else {
        image_thumbnail
    }
}

pub fn render_homepage(feed: MdHomepageFeed, is_localhost: bool) -> String {
    let popular_manga = feed.currently_popular;
    let new_chapters = feed.new_chapter_releases;
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="ressources/styles.css";
            script src = {"/ressources/index.js"}{}
            title  {"Home | MD_Light"}
            body {
            // h1 {"HOME"}
            (get_top_bar())
            div.popular{
                @for i in popular_manga{
                    div.popular-manga{
                        a href = (format!("/manga/{}",i.id)){
                            img src = (get_correct_image(is_localhost, i.cover))loading="lazy";
                            {(i.title)}
                        }
                    }
            }
            }
            div.new-release{
                @for chapter in new_chapters{
                    div.new_chapter{
                a.chapter href = (format!("/manga/{manga_id}/{chapter_id}", manga_id = chapter.manga_id, chapter_id = chapter.chapter_id)){
                    {(chapter.chapter_name) (to_flag_str(chapter.language.as_str()))}
                };
            }
                }
            }
        }
    );
    template.into_string()
}

// pub fn render_manga_info_page(manga_info: MangaInfo, is_localhost: bool) -> String {
//     println!("{}\n{}", manga_info.description, to_html(&manga_info.description));

//     let template = html!(
//         (DOCTYPE)
//         link rel="stylesheet" href="/ressources/styles.css";
//         script src = {"/ressources/index.js"}{}
//         title  {(manga_info.manga_name) " | MD_Light"}
//         // script type={"module"} src ={"https://md-block.verou.me/md-block.js"}{}

//         body{
//             (get_top_bar())
//             div.manga_info{
//                 div.col_1{
//                 img.cover_art src = (get_correct_image(is_localhost, manga_info.thumbnail));
//                 }
//                 div.col_2{
//                    h1  {(manga_info.manga_name)}

//                    @ let desc = to_html(&manga_info.description);
//                     // div.description{(desc)}
//                     div.description {(get_desc(desc))}

//                     // md-span.description{(manga_info.description)}
//                 }
//                 div.author_list{
//                     h3{"authors: "}
//                     @for author in manga_info.author{
//                          div.author{
//                              a href = {"/author/"(author.author_id)}{
//                                  {(author.role)": "(author.author_name)}
//                              }
//                          }

//                     };
//                 }

//         }
//         div.chapter_list{
//             @for chapter in manga_info.chapters{

//             // skips the chapter if it contains an error
//             @let chapter = match chapter {
//                 Ok(e)=> e,
//                 Err(_)=> continue,
//             };

//             div.chapter_item{
//                 a.chapter_link href = (format!("/manga/{manga_id}/{chapter}", manga_id = manga_info.manga_id, chapter = chapter.chapter_id )){
//                     // link to the chapter
//                 div.chapter{
//                     div.language{(get_flag_offline(&chapter.language.unwrap_or("N/a".to_owned())))" Ch."(chapter.chapter_number)}

//                        div.chapter_name{(chapter.chapter_name.unwrap_or(format!("Chapter {}", chapter.chapter_number)))}
//                     //     div.chapter_number{(format!("Chapter {}", chapter.chapter_number))}
//                     }
//                 }
//                 div.tl_group{
//                     // {(chapter.tl_group[0].name)}
//                 }
//             }
//             };
//         }
//         }
//     );
//     template.into_string()
// }

pub fn render_chapter(chapter_info: ChapterPage, is_localhost: bool, manga_id: String) -> String {
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="/ressources/styles.css";
            script src = {"/ressources/index.js"}{}
            // title  {(chapter_info.name)" | MD_Light"}

            body {
                (get_top_bar())
            // h1 {(chapter_info.name)}
            div.page_list ondblclick = "goFullscreen()";{
                @for i in chapter_info.pages{
                    img.chapter_page src = (get_correct_image(is_localhost, i));
                }
            }
            (get_return_to_manga(manga_id))
        }
    );
    template.into_string()
}

pub fn render_search_page(search_results: Vec<ShortMangaInfo>, is_localhost: bool) -> String {
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="/ressources/styles.css";
            script src = {"/ressources/index.js"}{}
            title  {"Search | MD_Light"}

            body {
                (get_top_bar())
            h1 {"search results"}
            div.search_list.works{
                @for i in search_results{
                    div.manga_restult.title{
                        a href = (format!("/manga/{}",i.id)){
                            img src = (get_correct_image(is_localhost, i.cover))loading="lazy";
                        div.manga-title{(i.title)}
                        }
                    }
                }
            }
        }
    );
    template.into_string()
}

// renders the error page and shows the user the
pub fn render_error_page(error_code: ApiError, requested_page: &str) -> String {
    // let probable_error_cause = todo!();
    println!("{}", error_code);
    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="/ressources/styles.css";
            title  {"Error! | MD_Light"}

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
        title  {(author.name)" | MD_Light"}
        body{
            (get_top_bar())

            div.author_name {(author.name)}
            // script {"console.log(`kj`)"}
            // div.title_number {(author.titles_id.len())" titles"}
            div.works #works{

                // button onclick = {"getAuhorChapters(['"(author.titles_id.join("', '"))"'])"}{}
                }
                script {"getAuhorChapters('"(author.id)"')"};
                // script {"getAuhorChapters(['"(author.titles_id.join("', '"))"'])"};

        }
    );
    template.into_string()
}

pub fn render_author_manga(titles: Vec<ShortMangaInfo>, is_localhost: bool) -> String {
    let template = html!(
            @for manga in titles{
    div.title{
        a.title-image href = {"/manga/"(manga.id)}{
            img src = (get_correct_image(is_localhost, manga.cover)) loading="lazy";
            div.manga-title{(manga.title)}
        }
    }



            }
        );
    template.into_string()
}

// renders the server config page
pub fn get_server_options() -> String {
    let template = html!(
        (DOCTYPE)
        (get_top_bar())
        title {"Server options | MD_Light"}
        body{

            button type="button" onclick = "location.href = '/server/kill' "{"Kill server"}
            button type="button" onclick = "location.href = '/server/ping' "{"Ping MangaDex"}


            a href = ("https://gitlab.com/_alexou_/md_light.git"){"pull via https:"};

        }
    );
    template.into_string()
}

pub fn render_complete_search(
    search_data: (Vec<ShortMangaInfo>, Vec<AuthorInfo>),
    is_localhost: bool,
    query: String,
) -> String {
    let search_results = search_data.0;
    let authors = search_data.1;

    let template = html!(
            (DOCTYPE)
            link rel="stylesheet" href="/ressources/styles.css";
            script src = {"/ressources/index.js"}{}
            title  {"Search | MD_Light"}

            body {
                (get_top_bar())
                h1 {"search " (query)}
            h2 {(search_results.len())" titles"}
            div.search_list.works{
                @for i in search_results{
                    div.manga_result.title{
                        a href = (format!("/manga/{}",i.id)){
                            img src = (get_correct_image(is_localhost, i.cover))loading="lazy";
                        div.manga-title{(i.title)}
                        }
                    }
                }
            }
            h2 {(authors.len())" authors"}
            div.search_author{
                @for i in authors{
                    div.author_result{
                        a href = {"/author/"(i.id)}{(i.name)};
                    }
                }
            }
        }
    );
    template.into_string()
}
