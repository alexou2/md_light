use serde_json::Value;
// use serde_derive::Serialize;
use crate::api_error::ApiError;
use serde::Serialize;
use crate::language::Language;

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
    pub name: String,
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
    pub name: String,
    pub id: String,
    // pub tags: Vec<String>,
    pub cover: String,
    pub status: String,
    pub original_language: String,
    pub translated_languages: Vec<Value>,
    pub description: String,
}

#[derive(Serialize)]
/// the complete informations about a specific manga
pub struct MangaInfo {
    pub manga_name: String,
    pub manga_id: String,
    pub author: Vec<Author>,
    pub tags: Vec<String>,
    pub thumbnail: String,
    pub status: String,
    pub original_language: String,
    pub translated_languages: Vec<Option<String>>,
    pub year: Option<i64>,
    pub description: String,
    // pub chapters: Vec<Result<Chapter, ApiError>>,
    // pub chapters: Vec<Result<Chapter, ApiError>>,
}

#[derive(Serialize)]
pub struct Author {
    pub author_name: String,
    pub author_id: String,
    pub role: String,
}

/// the chapters that are listed in the manga info page
#[derive(Clone, Serialize)]
pub struct Chapter {
    pub tl_group: Vec<TlGroup>,
    pub chapter_name: Option<String>,
    pub chapter_number: String,
    pub language: Language,
    pub chapter_id: String,
}
#[derive(Serialize)]
pub struct AuthorInfo {
    pub name: String,
    pub id: String,
    pub titles_id: Vec<String>,
}

#[derive(Clone, Serialize)]
pub struct TlGroup {
    pub id: String,
    pub name: String,
}

pub struct ChapterPage {
    pub chapter_name: String,
    pub pages: Vec<String>,
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

pub enum Source {
    MangaDex,
    Comick
}
