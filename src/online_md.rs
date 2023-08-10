// use std::{fmt::format, intrinsics::mir::BasicBlock};

use crate::md_struct::*;
use reqwest;
use serde;
use serde_json::{Value, json};
use std::error::Error;
use std::fmt::format;
use std::process::exit;

const BASE_URL: &'static str = "https://api.mangadex.org";

pub async fn test_connection() -> Result<String, reqwest::Error> {
    Ok(reqwest::get(format!("{}/ping", BASE_URL))
        .await?
        .text()
        .await?)
}

pub fn get_md_homepage_feed() -> MdHomepageFeed {
    todo!()
}
pub async fn search_manga(
    search_query: String,
) -> Result<Vec<MangaSearch>, Box<dyn std::error::Error>> {
    let mut search_results: Vec<MangaSearch> = Vec::new();
    let url = format!("{}/manga", BASE_URL); // formatting the correct url for the api endpoint

    let params = [("title", search_query)]; // setting the parameters of the search
    let client = reqwest::Client::new();
    let resp = client.get(url).query(&params).send().await?.text().await?;

    let json_resp: Value = serde_json::from_str(&resp)?; // converting the response string into a json object

    if let Some(response_data) = json_resp["data"].as_array() {
        for manga in response_data {
            let attributes = &manga["attributes"];

            let manga_id = &manga["id"];
            let title = &attributes["title"]["en"];
            let status = &attributes["status"];
            let original_language = &attributes["originalLanguage"];
            let available_languages = &attributes["availableTranslatedLanguages"]
                .as_array()
                .unwrap();
            // println!(
            //     "{} {} {} {} {:?}",
            //     manga_id, status, original_language, title, available_languages
            // );
            let thumbnail = get_manga_cover(manga_id).await?;
            let manga_attributes = MangaSearch {
                manga_name: title.clone(),
                manga_id: manga_id.clone(),
                thumbnail:thumbnail,
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

pub async fn get_manga_cover(manga_id: &Value) -> Result<String, Box<dyn Error>> {
    let url = format!("https://api.mangadex.org/cover?limit=10&manga%5B%5D={}&includes%5B%5D=manga", manga_id.to_string().replace('"', ""));
    let res = reqwest::get(url).await?.text().await?;
    let json_res: Value = serde_json::from_str(&res)?;
    let cover_file_name = &json_res["data"][0]["attributes"]["fileName"].to_string().replace('"', "");
    let cover_link = format!("https://uploads.mangadex.org/covers/{manga_id}/{cover_file_name}.256.jpg").replace('"', "");
    Ok(cover_link)
}
