
#[derive(serde::Deserialize)]
pub struct ChapterQuery {
    pub offset: i32,
    pub language: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub ordering: Option<String>
}

