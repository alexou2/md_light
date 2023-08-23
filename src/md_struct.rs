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
    pub chapter_name: String,
    pub chapter_number: String,
    pub language: String,
    pub chapter_id:String,
    pub manga_id: String,
    // pub thumbnail: String,
    pub tl_group_id:String,
    pub tl_group_name:String,
    pub page_number:String
}

pub struct MangaSearch {
    pub manga_name: String,
    pub manga_id: String,
    // pub tags: Vec<String>,
    pub thumbnail: String,
    pub status: String,
    pub original_language: String,
    pub translated_languages: Vec<Value>,
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
pub struct Chapters {
    // pub tl_group: String,
    pub chapter_name: String,
    pub chapter_number: String,
    pub language: String,
    pub chapter_id: String,
}
pub struct AuthorInfo {
    name: String,
    id: String,
    titles: Vec<MangaInfo>,
}
pub struct ChapterInfo{
    pub chapter_name:String,
    pub pages:Vec<String>
}

trait sort {
    fn sort_by_chapter(&self)-> Vec<Chapters>;
}
// sorts the chapters by chapter number and by translated language
impl sort for Chapters {
    fn sort_by_chapter(&self)-> Vec<Chapters> {
        todo!()
    }
}