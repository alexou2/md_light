use crate::api_error::ApiError;
use crate::cli_options::CliArgs;
use dirs;
use reqwest;
use std::{
    fs::{self, create_dir, read_dir, write},
    path::PathBuf,
};
use toml;

// base url of the repo
const BASE_REPO_URL: &'static str = "https://gitlab.com/_alexou_/md_light/-/raw/master/";

/// initialises the server by creating the required files for the server
pub fn init(args: &mut CliArgs) {
    args.install = false;
    println!("{}", args.install);
    create_config(args)
}

/// creates the ~.
fn create_config(args: &CliArgs) {
    let mut config_folder = dirs::config_local_dir().unwrap();
    config_folder.push("md_light");

    match &config_folder.exists() {
        true => (),
        false => create_dir(&config_folder).unwrap(),
    }

    // the path of the config file
    let config_file = init_config_file(args);
    let mut file_path: PathBuf = config_folder.clone();

    file_path.push("mdl.conf");
    fs::write(file_path, config_file).unwrap();
}

fn init_config_file(args: &CliArgs) -> String {
    let config_content = toml::to_string(args).unwrap();
    println!("{}", config_content);
    config_content
}

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
        "ressources/logo.svg",
        "ressources/feather.svg",
        "ressources/home.svg",
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
