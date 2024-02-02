/// used when loading the chapter for a manga
#[derive(serde::Deserialize)]
pub struct ChapterQuery {
    pub offset: i32,
    pub language: Option<String>,
}
/// used when searching for a manga or author
#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub ordering: Option<String>
}

