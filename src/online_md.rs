// use std::{fmt::format, intrinsics::mir::BasicBlock};

use crate::md_struct::*;
use crate::utills;
use chrono::Date;
use reqwest;
use serde_json::json;
use serde_json::{from_str, Value};
use std::{error::Error, fs::write};

const BASE_URL: &'static str = "https://api.mangadex.org";

pub async fn test_connection() -> Result<String, reqwest::Error> {
    Ok(reqwest::get(format!("{}/ping", BASE_URL))
        .await?
        .text()
        .await?)
}
// gets the informations for the homepage
pub async fn get_md_homepage_feed() -> Result<MdHomepageFeed, Box<dyn Error>> {
    let popular_manga = get_popular_manga().await?;
    let new_chapters: Vec<NewChapters> = Vec::new();

    // builds the struct for the popular titles+ new chapters
    let homepage_feed = MdHomepageFeed {
        currently_popular: popular_manga,
        new_chapter_releases: new_chapters,
    };
    Ok(homepage_feed)
}

async fn get_new_chapters() -> Result<Vec<NewChapters>, Box<dyn std::error::Error>> {
    let new_chapters = Vec::new();
    Ok(new_chapters)
}

// gets the most popular mangas from the last month that are displayed at the top of the md homepage
pub async fn get_popular_manga() -> Result<Vec<PopularManga>, Box<dyn std::error::Error>> {
    let mut popular_manga: Vec<PopularManga> = Vec::new();
    let formatted_time = utills::get_offset_time();
    let url = format!(
        r"https://api.mangadex.org/manga?includes[]=cover_art&includes[]=artist&includes[]=author&order[followedCount]=desc&contentRating[]=safe&contentRating[]=suggestive&hasAvailableChapters=true&createdAtSince={}",
        formatted_time
    );
    println!("{}", url);
    let resp = reqwest::get(url).await?.text().await?;
    let json_resp: Value = serde_json::from_str(&resp)?;

    if let Some(response_data) = json_resp["data"].as_array() {
        let mut ranking_number = 1;
        for manga in response_data {
            let title = &manga["attributes"]["title"]["en"]
                .to_string()
                .replace('"', "");
            let manga_id = &manga["id"].to_string().replace('"', "");
            let thumbnail = get_manga_cover(manga_id, manga)?;

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
    let url = format!("{}/manga?includes[]=cover_art", BASE_URL); // formatting the correct url for the api endpoint

    let params = [("title", search_query)]; // setting the parameters of the search
    let client: reqwest::Client = reqwest::Client::new();
    let resp = client.get(url).query(&params).send().await?.text().await?;
    let json_resp: Value = serde_json::from_str(&resp)?; // converting the response string into a json object

    if let Some(response_data) = json_resp["data"].as_array() {
        for manga in response_data {
            let attributes = &manga["attributes"];

            let manga_id = &manga["id"].to_string().replace('"', "");
            let title = &attributes["title"]["en"].to_string().replace('"', "");
            let status = &attributes["status"].to_string().replace('"', "");
            let original_language = &attributes["originalLanguage"].to_string().replace('"', "");
            let available_languages = &attributes["availableTranslatedLanguages"]
                .as_array()
                .ok_or("available languages is not an array")?;

            let thumbnail = get_manga_cover(manga_id, &manga)?;

            // creating the struct instnce containing all of the usefull ionfos about the manga
            let manga_attributes = MangaSearch {
                manga_name: title.clone(),
                manga_id: manga_id.clone(),
                thumbnail: thumbnail,
                status: status.clone(),
                original_language: original_language.clone(),
                translated_languages: available_languages.clone().clone(),
            };
            search_results.push(manga_attributes)
        }
    }
    Ok(search_results)
}

// gets the cover from the json
pub fn get_manga_cover(manga_id: &String, manga: &Value) -> Result<String, Box<dyn Error>> {
    let mut thumbnail = String::new();
    if let Some(manga_cover) = manga["relationships"].as_array() {
        for i in manga_cover {
            if i["type"] == "cover_art" {
                let cover_id = i["attributes"]["fileName"].to_string();
                let cover_link =
                    format!("https://mangadex.org/covers/{manga_id}/{cover_id}.256.jpg")
                        .replace('"', "");
                thumbnail = cover_link;
                break;
            }
        }
    }
    Ok(thumbnail)
}

pub async fn get_manga_info(manga_id: String) -> Result<MangaInfo, Box<dyn Error>> {
    let manga_chapters_promise = get_manga_chapters(&manga_id);
    let url = format!(
        "{}/manga/{}?includes[]=author&includes[]=artist&includes[]=cover_art",
        BASE_URL, &manga_id
    );
    let resp = reqwest::get(&url).await?.text().await?;
    let json_resp: Value = from_str(&resp)?;
    let data = &json_resp["data"];
    let attributes = &data["attributes"];
    // gets all of the infos about manga
    let manga_name = &attributes["title"]["en"].to_string().replace('"', "");
    let thumbnail = get_manga_cover(&manga_id, &data)?;
    let status = &attributes["status"].to_string().replace('"', "");
    let original_language = &attributes["originalLanguage"].to_string().replace('"', "");
    let description = &attributes["description"]["en"].to_string().replace('"', "");
    println!("{}", &attributes["year"]);
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
                let author_name = &author["attributes"]["name"].to_string().replace('"', "");
                let author_id = &author["id"].to_string().replace('"', "");
                let role = &author["type"].to_string().replace('"', "");
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

    let tag_json = attributes["tags"]
        .as_array()
        .ok_or("tags is not an array")?;

    for tag in tag_json {
        let tag_name = &tag["attributes"]["name"]["en"].to_string().replace('"', "");
        tag_list.push(tag_name.clone());
    }

    let translation_options_json = attributes["availableTranslatedLanguages"]
        .as_array()
        .ok_or("translated_languages is not an array")?;

    for language in translation_options_json {
        translated_language_list.push(language.to_string().replace('"', ""));
    }

    // building the struct
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
        chapters: manga_chapters_promise.await?,
    };
    Ok(manga_info)
}

pub async fn get_manga_chapters(manga_id: &String) -> Result<Vec<ChapterInfo>, Box<dyn Error>> {
    let url = format!("{}/manga/{}/feed", BASE_URL, manga_id);
    let resp = reqwest::get(&url).await?.text().await?;
    let json_resp: Value = from_str(&resp)?;
    let data = &json_resp["data"]; //transforming the response string into a json object
    let mut chapter_list: Vec<ChapterInfo> = Vec::new();
    let chapter_json = data.as_array().ok_or("there are no chapters")?; // transforming the json into an array
let mut  i = 0;
    for chapter in chapter_json {
        let attributes = &chapter["attributes"];
        // let tl_group = &manga["relationships"][""]
        let chapter_number = &attributes["chapter"].to_string().replace('"', "");
        // let chapter_name = format!("{number} {name}",number = chapter_number, name = &attributes["title"].to_string().replace('"', ""));
        let chapter_name = format!("Chapter {}", chapter_number.clone());
        let language = &attributes["translatedLanguage"].to_string().replace('"', "");
        let chapter_id = chapter["id"].to_string().replace('"', "");
        i+=1;
let chapter_instance = ChapterInfo{
    chapter_name:chapter_name,
    chapter_number:chapter_number.clone(),
    language:language.clone(),
    chapter_id:chapter_id,
    i:i
};
chapter_list.push(chapter_instance)
    }
    // chapter_list.push(ChapterInfo {chapter_name: "ch1".to_string(), chapter_number: "1".to_string(), language: "en".to_string(), chapter_id: "123".to_string() });
    Ok(chapter_list)
}
