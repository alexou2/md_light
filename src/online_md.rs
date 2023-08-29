// use std::{fmt::format, intrinsics::mir::BasicBlock};

use crate::md_struct::*;
use crate::utills::*;
use reqwest;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use serde_json::{from_str, json, Value};
use std::future::Future;
use std::{error::Error, fs::write};

const BASE_URL: &'static str = "https://api.mangadex.org";

// sends a get request to the /ping endpoint of the api
pub async fn test_connection() -> Result<String, reqwest::Error> {
    Ok(reqwest::get(format!("{}/ping", BASE_URL))
        .await?
        .text()
        .await?)
}

// makes the request to the url with custom user agents, since MD requires them now
pub async fn request_with_agent(
    url: String,
) -> Result<impl Future<Output = Result<reqwest::Response, reqwest::Error>>, Box<dyn Error>> {
    let client: Client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send();

    Ok(response)
}

// gets the informations for the homepage
pub async fn get_md_homepage_feed() -> Result<MdHomepageFeed, Box<dyn Error>> {
    let popular_manga = get_popular_manga();
    let new_chapters = get_new_chapters();

    // builds the struct for the popular titles+ new chapters
    let homepage_feed = MdHomepageFeed {
        currently_popular: popular_manga.await?,
        new_chapter_releases: new_chapters.await?,
    };

    Ok(homepage_feed)
}

// gets the new chapter releases for the homepage
pub async fn get_new_chapters() -> Result<Vec<NewChapters>, Box<dyn std::error::Error>> {
    let mut new_chapters = Vec::new();
    let url = "https://api.mangadex.org/chapter?includes[]=scanlation_group&translatedLanguage[]=en&translatedLanguage[]=de&contentRating[]=safe&contentRating[]=suggestive&contentRating[]=erotica&order[readableAt]=desc&limit=64&includes[]=cover_art";
    let resp = request_with_agent(url.to_string())
        .await?
        .await?
        .text()
        .await?;
    // converts the api response to a json string and gets the data part of it
    let json_resp: Value = from_str(&resp)?;
    let data = &json_resp["data"];

    // getting the required info of each new chapter
    if let Some(new_chapter_list) = data.as_array() {
        // transforming the json data into an array
        for chapter in new_chapter_list {
            let attributes = &chapter["attributes"];
            let relationships = &chapter["relationships"];

            let chapter_id = &chapter["id"]
                .remove_quotes()
                .ok_or("error while removing quotes")?;
            let chapter_number = attributes["chapter"]
                .remove_quotes()
                .unwrap_or("Oneshot".to_string()); //sets the chapter number to Oneshot if the real number is null
            let chapter_name = &attributes["title"]
                .remove_quotes()
                .unwrap_or(chapter_number.clone()); // sets the chapter name to the chapter number if the real chapter name is null
            let language = &attributes["translatedLanguage"]
                .remove_quotes()
                .ok_or("error while removing quotes in language")?;
            let page_number = attributes["pages"]
                .remove_quotes()
                .ok_or("error while removing quotes in page")?;

            let mut tl_group_id: String = String::new();
            let mut tl_group_name: String = String::new();
            let mut manga_id: String = String::new();

            // gets the infos about the tl group and the mangaID
            if let Some(relation) = relationships.as_array() {
                for rel in relation {
                    // matches the tl group or the manga_id and sets the value to the correct variable
                    match rel["type"].to_string().replace('"', "").as_str() {
                        "scanlation_group" => {
                            tl_group_id = rel["id"]
                                .remove_quotes()
                                .ok_or("error while removing quotes")?;
                            tl_group_name = rel["attributes"]["name"]
                                .remove_quotes()
                                .ok_or("error while removing quotes")?;
                        }
                        "manga" => {
                            manga_id = rel["id"]
                                .remove_quotes()
                                .ok_or("error while removing quotes")?
                        }
                        _ => continue,
                    }
                }
            }

            let new_chapter_data = NewChapters {
                chapter_name: chapter_name.to_owned(),
                chapter_number: chapter_number,
                language: language.to_owned(),
                // thumbnail: "N/A".to_string(),
                manga_id: manga_id,
                tl_group_id: tl_group_id,
                tl_group_name: tl_group_name,
                chapter_id: chapter_id.to_owned(),
                page_number: page_number,
            };
            new_chapters.push(new_chapter_data)
        }
    }

    Ok(new_chapters)
}

// gets the most popular mangas from the last month that are displayed at the top of the md homepage
pub async fn get_popular_manga() -> Result<Vec<PopularManga>, Box<dyn std::error::Error>> {
    let mut popular_manga: Vec<PopularManga> = Vec::new();
    let formatted_time = get_offset_time();
    // formatting the request url to include the atrists/authores and the cover fileName
    let url = format!(
        r"https://api.mangadex.org/manga?includes[]=cover_art&includes[]=artist&includes[]=author&order[followedCount]=desc&contentRating[]=safe&contentRating[]=suggestive&hasAvailableChapters=true&createdAtSince={}",
        formatted_time
    );
    // doing the get request to the api and transforming it into a json object
    let resp = request_with_agent(url).await?.await?.text().await?;
    let json_resp: Value = serde_json::from_str(&resp)?;
    write("t.json", resp);
    // transforming the json into an array in order to get all of the search results
    if let Some(response_data) = json_resp["data"].as_array() {
        for manga in response_data {
            println!("{}", manga);
            let title = manga["attributes"]["title"]
                .as_object()
                .and_then(|obj| obj.values().next())
                .ok_or("error while getting manga title")?
                .remove_quotes()
                .ok_or("error while removing quotes")?;
            let manga_id = &manga["id"]
                .remove_quotes()
                .ok_or("error while removing quotes")?;
            let thumbnail = get_manga_cover(manga_id, manga)?;

            // creating the search result for each popular manga
            let manga_instance = PopularManga {
                manga_name: title.clone(),
                thumbnail: thumbnail.clone(),
                manga_id: manga_id.clone(),
            };

            popular_manga.push(manga_instance);
        }
    }
    Ok(popular_manga)
}

// searches for a manga
pub async fn search_manga(
    search_query: String,
) -> Result<Vec<MangaSearch>, Box<dyn std::error::Error>> {
    let mut search_results: Vec<MangaSearch> = Vec::new();
    // sending the get request for the search
    let url = format!("{}/manga?includes[]=cover_art", BASE_URL); // formatting the correct url for the api endpoint
    let params = [("title", search_query)]; // setting the parameters of the search
    let client: reqwest::Client = reqwest::Client::new();
    let resp = client
        .get(url)
        .query(&params)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()
        .await?
        .text()
        .await?;

    // converting the response string into a json object
    let json_resp: Value = serde_json::from_str(&resp)?;
    // write("t.json", resp);
    // if the api response is an array, add every manga to the search_results vector
    if let Some(response_data) = json_resp["data"].as_array() {
        for manga in response_data {
            let attributes = &manga["attributes"];

            // getting eery necessary info from the manga
            let manga_id = &manga["id"]
                .remove_quotes()
                .ok_or("error while removing quotes")?;

            let title: String = attributes["title"]
                .as_object()
                .and_then(|obj| obj.values().next())
                .ok_or("error while getting title")?
                .remove_quotes()
                .ok_or("error while removing quotes")?;
            let status = &attributes["status"]
                .remove_quotes()
                .ok_or("error while removing quotes")?;
            let original_language = &attributes["originalLanguage"]
                .remove_quotes()
                .ok_or("error while removing quotes")?;
            let available_languages = attributes["availableTranslatedLanguages"]
                .as_array()
                .ok_or("error while getting translated languages options")?;
            let thumbnail = get_manga_cover(manga_id, &manga)?;

            // creating the struct instnce containing all of the usefull ionfos about the manga
            let manga_attributes = MangaSearch {
                manga_name: title.clone(),
                manga_id: manga_id.clone(),
                thumbnail: thumbnail,
                status: status.clone(),
                original_language: original_language.clone(),
                translated_languages: available_languages.clone(),
            };
            search_results.push(manga_attributes)
        }
    }
    Ok(search_results)
}

// gets the cover from the json the request url needs to have includes[]=cover_art for this function to work
pub fn get_manga_cover(manga_id: &String, manga_json: &Value) -> Result<String, Box<dyn Error>> {
    let mut thumbnail = String::new();
    if let Some(manga_cover) = manga_json["relationships"].as_array() {
        for i in manga_cover {
            if i["type"] == "cover_art" {
                let cover_id = i["attributes"]["fileName"].to_string();
                let cover_link =
                    format!("https://mangadex.org/covers/{manga_id}/{cover_id}.256.jpg")
                        .replace('"', "");
                thumbnail = cover_link;
                break; //breaks the loop if the cover is found
            }
        }
    }
    Ok(thumbnail)
}

// gets the manga info and chapters
pub async fn get_manga_info(manga_id: String) -> Result<MangaInfo, Box<dyn Error>> {
    // calls the function to get chapters for a faster page loading
    let manga_chapters_promise = get_manga_chapters(&manga_id);

    let url = format!(
        "{}/manga/{}?includes[]=author&includes[]=artist&includes[]=cover_art",
        BASE_URL, &manga_id
    );
    // calling the function to make the request to the api
    let resp = request_with_agent(url).await?.await?.text().await?;

    // parsing the api response into a json
    let json_resp: Value = from_str(&resp)?;
    // separating the json response to make it easier to access items
    let data = &json_resp["data"];
    let attributes = &data["attributes"];

    // gets all of the infos about manga
    let manga_name = attributes["title"]
        .as_object()
        .and_then(|obj| obj.values().next())
        .ok_or("error while getting title")?
        .remove_quotes()
        .ok_or("error while removing quotes in the manga name")?;
    let thumbnail = get_manga_cover(&manga_id, &data)?;
    let status = &attributes["status"]
        .remove_quotes()
        .ok_or("error while removing quotes in the status")?;
    let original_language = &attributes["originalLanguage"]
        .remove_quotes()
        .ok_or("error while removing quotes in the og language")?;
    println!("{}", &attributes["description"]);
    let description = &attributes["description"]["en"]
        .remove_quotes()
        .unwrap_or("N/a".to_string());
    let year = &attributes["year"].as_i64();

    let mut author_list: Vec<Author> = Vec::new();
    let mut tag_list = Vec::new();
    let mut translated_language_list = Vec::new();

    // transforming the json part containing the authors into an array
    let author_json = data["relationships"]
        .as_array()
        .ok_or("authors is not an array")?;

    // gets the list pf authors involved in the manga
    for author in author_json {
        // breaks the loop if the data isn't about an author/drawer
        match author["type"].to_string().replace('"', "").as_str() {
            "author" | "artist" => {
                let author_name = &author["attributes"]["name"]
                    .remove_quotes()
                    .ok_or("error while removing quotes in the author name")?;
                let author_id = &author["id"]
                    .remove_quotes()
                    .ok_or("error while removing quotes for the author ID")?;
                let role = &author["type"]
                    .remove_quotes()
                    .ok_or("error while removing quotes in the author role")?;

                // the author/artist instance
                let author_instance = Author {
                    author_name: author_name.clone(),
                    author_id: author_id.clone(),
                    role: role.clone(),
                };
                author_list.push(author_instance)
            }
            _ => break,
        }
    }

    // getting the tags
    let tag_json = attributes["tags"]
        .as_array()
        .ok_or("tags is not an array")?;
    for tag in tag_json {
        let tag_name = &tag["attributes"]["name"]["en"]
            .remove_quotes()
            .ok_or("error while removing quotes in the tags")?;
        tag_list.push(tag_name.clone());
    }

    // getting the translation options
    let translation_options_json = attributes["availableTranslatedLanguages"]
        .as_array()
        .ok_or("translated_languages is not an array")?;
    for language in translation_options_json {
        translated_language_list.push(
            language
                .remove_quotes()
                .ok_or("error while removing quotes in the language options")?,
        );
    }

    // building the struct with all of the manga's informations+ chapters
    let manga_info = MangaInfo {
        manga_name: manga_name.clone(),
        manga_id: manga_id.clone(),
        author: author_list,
        tags: tag_list,
        thumbnail: thumbnail,
        status: status.clone(),
        original_language: original_language.clone(),
        translated_languages: translated_language_list,
        year: year.clone(),
        description: description.clone(),
        chapters: sort_by_chapter(manga_chapters_promise.await?), // waits until the request to get the chapters is done
    };
    Ok(manga_info)
}

pub async fn get_manga_chapters(manga_id: &String) -> Result<Vec<Chapters>, Box<dyn Error>> {
    let url = format!("{}/manga/{}/feed", BASE_URL, manga_id);
    // let resp = reqwest::get(&url).await?.text().await?;
    let resp = request_with_agent(url).await?.await?.text().await?;

    let json_resp: Value = from_str(&resp)?;
    let data = &json_resp["data"]; //transforming the response string into a json object
    let mut chapter_list: Vec<Chapters> = Vec::new();
    let chapter_json = data.as_array().ok_or("there are no chapters")?; // transforming the json into an array
    for chapter in chapter_json {
        let attributes = &chapter["attributes"];
        // let tl_group = &manga["relationships"][""]
        let chapter_number = &attributes["chapter"]
            .remove_quotes()
            .ok_or("error while removing quotes in the chapter number")?;
        // let chapter_name = format!("{number} {name}",number = chapter_number, name = &attributes["title"].to_string().replace('"', ""));
        let chapter_name = format!("Chapter {}", chapter_number.clone());
        let language = &attributes["translatedLanguage"]
            .remove_quotes()
            .ok_or("error while removing quotes in the chapter language")?;
        let chapter_id = chapter["id"]
            .remove_quotes()
            .ok_or("error while removing quotes in the chapter ID")?;
        let chapter_instance = Chapters {
            chapter_name: chapter_name.clone(),
            chapter_number: chapter_number.clone(),
            language: language.clone(),
            chapter_id: chapter_id,
        };
        chapter_list.push(chapter_instance)
    }
    Ok(chapter_list)
}

pub async fn get_chapter_pages(chapter_id: String) -> Result<ChapterInfo, Box<dyn Error>> {
    let url = format!("{}/at-home/server/{}", BASE_URL, chapter_id);
    // let resp = reqwest::get(&url).await?.text().await?;
    let resp = request_with_agent(url).await?.await?.text().await?;

    let json_resp: Value = from_str(&resp)?;
    // write("l.json", resp);
    let chapter_hash = json_resp["chapter"]["hash"].to_string().replace('"', "");
    let pages_json = json_resp["chapter"]["data"]
        .as_array()
        .ok_or("there are no pages")?; //transforming the response string into a json object

    let mut page_list: Vec<String> = Vec::new();
    for page in pages_json {
        let mut page_link = page
            .remove_quotes()
            .ok_or("error while removing quotes")?
            .to_string();
        page_link = format!(
            "https://uploads.mangadex.org/data/{}/{}",
            chapter_hash, page_link
        );
        page_list.push(page_link)
    }
    let chapter = ChapterInfo {
        chapter_name: "ch".to_string(),
        pages: page_list,
    };
    Ok(chapter)
}
