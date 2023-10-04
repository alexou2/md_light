use crate::api_error::ApiError;
use crate::md_struct::*;
use crate::online_md;
use crate::utills;
use reqwest;
use serde_json::json;
use serde_json::Value;
use std::fs::{create_dir, read_dir, write};
use toml;
use std::thread::spawn;

pub fn write_toml(offline_data: OfflineData) {
    let toml_str = toml::to_string(&offline_data).expect("unable to save");
    println!("{}", toml_str);
    write("employee_data.toml", toml_str).expect("Error writing file");
}

pub async fn downlad_manga(manga_id: String, downloaded_lang: String) -> Result<(), ApiError> {
    let manga_data = online_md::get_manga_info(manga_id).await?;

    let offline_data = OfflineData {
        name: manga_data.manga_name.clone(),
        manga_id: manga_data.manga_id.clone(),
        downloaded_lang: downloaded_lang,
        downloaded_at: chrono::Local::now().to_string(),
        tags: manga_data.tags,
        authors: manga_data.author,
        original_lang: manga_data.original_language.clone(),
        status: manga_data.status.clone(),
        description: manga_data.description.clone(),
        downloaded_chap: manga_data.chapters.len() as i32,
    };
    Ok(())
}

fn create_download_path() {}
fn save_image() {}
