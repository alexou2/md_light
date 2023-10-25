use reqwest;
use std::fs::{create_dir, read_dir, write};

use crate::api_error::ApiError;

pub async fn install_ressources() -> Result<(), ApiError> {
    let css_code = download_css().await?;
    let js_code = download_js().await?;

    let mut ressource_dir_present = false;
    let dir = "./";
    for entry in read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && path.to_string_lossy()== "./ressources" {
            ressource_dir_present = true;
            break;
        };
    }

    if !ressource_dir_present {
        create_dir("./ressources")?;
    }
    write("./ressources/styles.css", css_code)?;
    write("./ressources/index.js", js_code)?;

    Ok(())
}
async fn download_js() -> Result<String, ApiError> {
    let js_code =
        reqwest::get("https://gitlab.com/_alexou_/md_light/-/raw/master/ressources/index.js")
            .await?
            .text()
            .await?;
    Ok(js_code)
}
async fn download_css() -> Result<String, ApiError> {
    let css_code =
        reqwest::get("https://gitlab.com/_alexou_/md_light/-/raw/master/ressources/styles.css")
            .await?
            .text()
            .await?;
    Ok(css_code)
}
