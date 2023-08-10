use serde_json::Value;
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
}

pub struct MangaSearch {
    pub manga_name: Value,
    pub manga_id:Value,
    // pub tags: Vec<Value>,
    pub thumbnail: String,
    pub status: Value,
    pub original_language: Value,
    pub translated_languages: Vec<Value>,
}
