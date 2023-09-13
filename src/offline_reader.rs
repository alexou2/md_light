use crate::md_struct::*;
use crate::online_md;
use crate::utills;
use reqwest;
use serde_json::json;
use serde_json::Value;
use std::fs::{create_dir, read_dir, write};
use std::path;
use std::path::PathBuf;
// fn save_page(image_data: Vec<&str>, path: PathBuf) -> Result<(), std::fs::> {
//     create_dir(path);
//     for i in 0..image_data.len() {
//         let image_path = path.join(format!("page {}",i+1));
//         // .to_str().ok_or("error while formatting path to download image")?;
//         write(image_data[i], image_path);
//     }
//     Ok(())
// }
pub async fn download_manga(manga_id: String) {
    let manga_data = online_md::get_manga_info(manga_id).await;
}
pub async fn write_json(
    manga_data: MangaInfo,
    manga_directory: &str  ,
    downloaded_language: &str,
) {
    let mut chapter_json = Vec::new();
    for chapter in manga_data.chapters {
        let json = json!({
        "path":format!("{}/{}", manga_directory, chapter.chapter_name.clone().ok_or(&chapter.chapter_number).expect("unable to write the chapter name to the json file (the chapter name is null)")),
        "number":chapter.chapter_number,
        "name":chapter.chapter_name,
        "language":chapter.language,
        "chapter_id":chapter.chapter_id,
        "bookmarked":false,
        "read":false,
        });
        chapter_json.push(json)
    }

    let mut data = json!({
        "manga_id":manga_data.manga_id,
        "name":manga_data.manga_name,
        "thumbnail":manga_data.thumbnail,
        "status":manga_data.status,
        "original_language":manga_data.original_language,
        "description":manga_data.description,
        "year":manga_data.year,
        "downloaded_language":downloaded_language,
        "downloaded_at":utills::get_current_time(),
        "chapters":[
            chapter_json
        ]
    });
    // writing the file to the filesystem
    write(format!("{}.json", manga_data.manga_name), data.to_string());
}
