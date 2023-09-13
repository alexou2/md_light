use serde_json::{value::Serializer, Value};
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
// used in /manga/{id}
pub struct MangaInfo {
    pub manga_name: String,
    pub manga_id: String,
    pub author: Vec<Author>,
    pub tags: Vec<String>,
    pub thumbnail: String,
    pub status: String,
    pub original_language: String,
    pub translated_languages: Vec<String>,
    pub year: Option<i64>,
    pub description: String,
    pub chapters: Vec<Chapters>,
}
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
    pub language: String,
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
    pub id: String,
    pub name: String,
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
            println!("{}", self);
            None
        }
    }
}
