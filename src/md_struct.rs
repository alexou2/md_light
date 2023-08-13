use serde_json::Value;
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
    pub name: String,
    pub chapter: Chapters,
    pub language: String,
    pub manga_id: String,
    pub thumbnail: String,
    pub original_language: String,
}

pub struct MangaSearch {
    pub manga_name: String,
    pub manga_id: String,
    // pub tags: Vec<Value>,
    pub thumbnail: String,
    pub status: String,
    pub original_language: String,
    pub translated_languages: Vec<Value>,
}
// used in /manga/{id}
pub struct MangaInfo {
    pub manga_name: String,
    pub manga_id: String,
    pub author: Option<Vec<Author>>,
    pub tags: Option<Vec<String>>,
    pub thumbnail: String,
    pub status: String,
    pub original_language: String,
    pub translated_languages: Vec<String>,
    pub year: Option<i64>,
    pub description: Option<String>,
    pub chapters: Vec<Chapters>,
}
pub struct Author {
    pub author_name: String,
    pub author_id: String,
    pub role: String,
}
pub struct Chapters {
    // pub tl_group: String,
    pub chapter_name: String,
    pub chapter_number: String,
    pub language: String,
    pub chapter_id: String,
    pub i:i32
}
pub struct AuthorInfo {
    name: Value,
    id: Value,
    titles: Vec<MangaInfo>,
}
pub struct ChapterInfo{
    pub chapter_name:String,
    pub pages:Vec<String>
}