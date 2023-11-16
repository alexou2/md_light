use serde_json::Value;
use serde_derive::Deserialize;
// use serde_derive::Serialize;
use serde::{Serialize, Serializer};
use crate::api_error::ApiError;
use serde::ser::SerializeStruct;

pub struct MdHomepageFeed {
    pub currently_popular: Vec<PopularManga>,
    pub new_chapter_releases: Vec<NewChapters>,
}
pub struct PopularManga {
    pub manga_name: String,
    // pub description: String,
    pub thumbnail: String,
    pub manga_id: String,
}
pub struct NewChapters {
    pub chapter_name: String,
    pub chapter_number: String,
    pub language: String,
    pub chapter_id: String,
    pub manga_id: String,
    // pub thumbnail: String,
    pub tl_group_id: String,
    pub tl_group_name: String,
    pub page_number: String,
}
#[derive(Debug)]
pub struct ShortMangaInfo {
    pub manga_name: String,
    pub manga_id: String,
    // pub tags: Vec<String>,
    pub thumbnail: String,
    pub status: String,
    pub original_language: String,
    pub translated_languages: Vec<Value>,
    pub description: String,
}

impl Serialize for ShortMangaInfo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ShortMangaInfo", 7)?;
        state.serialize_field("manga_name", &self.manga_name)?;
        state.serialize_field("manga_id", &self.manga_id)?;
        // state.serialize_field("tags", &self.tags)?;
        state.serialize_field("thumbnail", &self.thumbnail)?;
        state.serialize_field("status", &self.status)?;
        state.serialize_field("original_language", &self.original_language)?;
        state.serialize_field("translated_languages", &self.translated_languages)?;
        state.serialize_field("description", &self.description)?;
        state.end()
    }
}


// used in /manga/{id}
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
    pub chapters: Vec<Result<Chapters, ApiError>>,
}
// #[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub author_name: String,
    pub author_id: String,
    pub role: String,
}
// the chapters that are listed in the manga info page
#[derive(Clone)]
pub struct Chapters {
    pub tl_group: Vec<TlGroup>,
    pub chapter_name: Option<String>,
    pub chapter_number: String,
    pub language: Option<String>,
    pub chapter_id: String,
}
pub struct AuthorInfo {
    pub name: String,
    pub id: String,
    // pub titles: Vec<ShortMangaInfo>,
    pub titles_id: Vec<String>,
}

#[derive(Clone)]
pub struct TlGroup {
    pub id:  String,
    pub name:  String,
}

pub struct ChapterPages {
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

/// structs that will be used to write the files when downloading the manga
// #[derive(Debug, Serialize, Deserialize)]
pub struct OfflineData {
    pub name: String,
    pub manga_id: String,
    pub downloaded_lang: String,
    pub downloaded_at: String, // the date at which the manga was last downloaded
    pub tags: Vec<String>,
    pub authors: Vec<Author>,
    pub original_lang:String,
    pub status:String,
    pub description:String,
    pub downloaded_chap:i32, // the number of downloaded chapters
}