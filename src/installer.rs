use reqwest;
use std::fs::{create_dir, read_dir, write};

use crate::api_error::ApiError;
const BASE_REPO_URL: &'static str = "https://gitlab.com/_alexou_/md_light/-/raw/master/";

pub async fn install_ressources() -> Result<(), ApiError> {
    // let css_code = download_css().await?;
    // let js_code = download_js().await?;

    // checks if the ressources directory is present
    let mut ressource_dir_present = false;
    let dir = "./";
    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.to_string_lossy() == "./ressources" {
            ressource_dir_present = true;
            break;
        };
    }
    // creates the directory if not present
    if !ressource_dir_present {
        create_dir("./ressources")?;
    }
    // the list of files to download
    let ressource_file_list = vec![
        "ressources/index.js",
        "ressources/styles.css",
        "ressources/profile.svg",
    ];
    // downloading and saving the files from the repo
    for file in ressource_file_list {
        let buffer = download_file(file).await?;
        write(file, buffer)?;
    }
    Ok(())
}
async fn download_file(file_path: &str) -> Result<String, ApiError> {
    let url = format!("{BASE_REPO_URL}{file_path}");

    let js_code = reqwest::get(url).await?.text().await?;
    Ok(js_code)
}
