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
    pub chpter: String,
    pub language: String,
    pub chapter_link: String,
    pub manga_link: String,
    pub thumbnail: String,
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
    pub manga_name: Value,
    pub manga_id: Value,
    pub author: Vec<Author>,

    pub tags: Vec<Value>,
    pub thumbnail: String,
    pub status: Value,
    pub original_language: Value,
    pub translated_languages: Vec<Value>,
    pub chapters: Vec<ChapterInfo>,
}
pub struct Author {
    pub author_name: Value,
    pub author_id: Value,
}
pub struct ChapterInfo {
    pub tl_group: Value,
    pub chapter_name: String,
    pub chapter_id: Value,
}
pub struct AuthorInfo {
    name: Value,
    id: Value,
    titles: Vec<MangaInfo>,
}
