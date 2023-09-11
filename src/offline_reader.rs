use crate::md_struct::*;
use crate::online_md;
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
pub async fn write_json(manga_data: MangaInfo) {
    // let manga_id = manga_data.manga_id;
    let data = json!({
        "manga_id":manga_data.manga_id,
        "name":manga_data.manga_name,
    });
    write(format!("{}.json", manga_data.manga_name), data.to_string());
}
