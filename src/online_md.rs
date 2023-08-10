// use std::{fmt::format, intrinsics::mir::BasicBlock};

use crate::md_struct::*;
use crate::utills;
use chrono::Date;
use reqwest;
use serde_json::json;
use serde_json::{from_str, Value};
use std::{error::Error, fmt::format};

const BASE_URL: &'static str = "https://api.mangadex.org";

pub async fn test_connection() -> Result<String, reqwest::Error> {
    Ok(reqwest::get(format!("{}/ping", BASE_URL))
        .await?
        .text()
        .await?)
}

pub async fn get_md_homepage_feed() -> Result<MdHomepageFeed, Box<dyn Error>> {
    let popular_manga = get_popular_manga().await?;
    let new_chapters: Vec<NewChapters> = Vec::new();

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
            let title = &manga["attributes"]["title"]["en"].to_string().replace('"', "");
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
                .unwrap();

            let thumbnail = get_manga_cover(manga_id, &manga)?;

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
    // std::fs::write("t.json", resp);
    Ok(search_results)
}

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
