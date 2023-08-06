// use std::{fmt::format, intrinsics::mir::BasicBlock};

use reqwest::ClientBuilder;
use serde;
use serde_json::json;

pub struct Timestamp(pub u64);
pub struct MdHomepageFeed {
    pub currently_popular: Vec<PopularManga>,
    pub new_chapter_releases: Vec<NewChapters>,
}
pub struct PopularManga {
    pub name: String,
    pub description: String,
    pub thumbnail_link: String,
}
pub struct NewChapters {
    pub name: String,
    pub chpter: String,
    pub language: String,
    pub chapter_link: String,
    pub manga_link: String,
    pub thumbnail: String,
    pub time_of_post: Timestamp
}
const BASE_URL: &'static str = "https://api.mangadex.org";

pub async fn test_connection()-> Result<String, reqwest::Error>{
Ok(reqwest::get(format!("{}/ping", BASE_URL)).await?.text().await?)
}


pub fn get_md_homepage_feed() {








}
