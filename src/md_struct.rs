use serde_json::Value;
pub struct MdHomepageFeed {
    pub currently_popular: Vec<PopularManga>,
    pub new_chapter_releases: Vec<NewChapters>,
}
pub struct PopularManga {
    pub manga_name: Value,
    // pub description: Value,
    pub thumbnail: Value,
    pub manga_id: Value,
}
pub struct NewChapters {
    pub chapter_name: Value,
    pub chapter_number: i32,
    pub language: Value,
    pub chapter_id:Value,
    pub manga_id: Value,
    // pub thumbnail: Value,
    pub tl_group_id:Value,
    pub tl_group_name:Value,
    pub page_number:i32
}

pub struct MangaSearch {
    pub manga_name: Value,
    pub manga_id: Value,
    // pub tags: Vec<Value>,
    pub thumbnail: Value,
    pub status: Value,
    pub original_language: Value,
    pub translated_languages: Vec<Value>,
}
// used in /manga/{id}
pub struct MangaInfo {
    pub manga_name: Value,
    pub manga_id: Value,
    pub author: Vec<Author>,
    pub tags: Vec<Value>,
    pub thumbnail: Value,
    pub status: Value,
    pub original_language: Value,
    pub translated_languages: Vec<Value>,
    pub year: Option<i64>,
    pub description: Value,
    pub chapters: Vec<Chapters>,
}
pub struct Author {
    pub author_name: Value,
    pub author_id: Value,
    pub role: Value,
}
pub struct Chapters {
    // pub tl_group: Value,
    pub chapter_name: Value,
    pub chapter_number: Value,
    pub language: Value,
    pub chapter_id: Value,
}
pub struct AuthorInfo {
    name: Value,
    id: Value,
    titles: Vec<MangaInfo>,
}
pub struct ChapterInfo{
    pub chapter_name:Value,
    pub pages:Vec<Value>
}