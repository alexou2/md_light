use serde_json::Value;
// use serde_derive::Serialize;
use crate::api_error::ApiError;
use crate::language::Language;
use serde::Serialize;

/// used when checking the mangadex api status from /server/ping
pub struct ServerStatus {
    /// is the server down for maintenance
    pub up: bool,
    /// is the server unreachable
    pub reachable: bool,
}
#[derive(Serialize)]
pub struct MdHomepageFeed {
    pub currently_popular: Vec<PopularManga>,
    pub new_chapter_releases: Vec<NewChapters>,
}

#[derive(Serialize)]
pub struct PopularManga {
    pub title: String,
    pub cover: String,
    pub id: String,
}

#[derive(Serialize)]
pub struct NewChapters {
    pub chapter_name: String,
    pub chapter_number: String,
    // pub language: String,
    pub language: Language,
    pub chapter_id: String,
    pub manga_id: String,
    pub tl_group_id: String,
    pub tl_group_name: String,
    pub page_number: String,
}

#[derive(Serialize)]
pub struct ShortMangaInfo {
    pub title: String,
    pub id: String,
    // pub tags: Vec<String>,
    pub cover: String,
    pub status: String,
    pub original_language: Language,
    pub translated_languages: Vec<Language>,
    pub description: String,
}

#[derive(Serialize)]
/// the complete informations about a specific manga
pub struct MangaInfo {
    pub manga_name: String,
    pub manga_id: String,
    pub author: Vec<Author>,
    pub tags: Vec<String>,
    pub cover: String,
    pub status: String,
    pub original_language: Language,
    pub translated_languages: Vec<Language>,
    pub year: Option<i64>,
    pub description: String,
    // pub chapters: Vec<Result<Chapter, ApiError>>,
}

#[derive(Serialize)]
pub struct Author {
    pub author_name: String,
    pub author_id: String,
    pub role: String,
}

/// the chapters that are listed in the manga info page
#[derive(Clone, Serialize, Debug)]
pub struct Chapter {
    pub tl_group: Vec<TlGroup>,
    pub chapter_name: Option<String>,
    pub chapter_number: String,
    pub language: Language,
    pub chapter_id: String,
}
#[derive(Clone, Serialize, Debug)]
pub struct CurrentChapter {
    pub curr_chapter_number: f32,
    pub curr_chapter_name: Option<String>,
    pub prev: Option<Chapter>,
    pub next: Option<Chapter>,
}

pub struct MangaChapters {
    pub chapters: Vec<Result<Chapter, ApiError>>,
    pub total: i32,
}

#[derive(Serialize)]
pub struct AuthorInfo {
    pub name: String,
    pub id: String,
    pub titles_id: Vec<String>,
}

#[derive(Clone, Serialize, Debug)]
pub struct TlGroup {
    pub id: String,
    pub name: String,
}
#[derive(Clone, Serialize)]
pub struct ChapterPage {
    pub name: String,
    pub pages: Vec<String>,
}

enum MangaImage{
    /// the url to the image
    Url(String),
    /// the data of the image. used when the 'embeded' argument is given
    Data(String)
}

pub trait ValueExtensions {
    fn remove_quotes(&self) -> Option<String>;
}
impl ValueExtensions for Value {
    fn remove_quotes(&self) -> Option<String> {
        if let Value::String(inner_value) = self {
            Some(inner_value.to_string())
        } else if self.is_number() {
            Some(self.to_string())
        } else {
            None
        }
    }
}
