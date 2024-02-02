use crate::md_struct::*;
use chrono::{DateTime, Datelike, Timelike, Utc};
use std::fs::{read_dir, write, DirBuilder};
/// these are the naming conventions that will be used to store the manga and its chapters
/// the manga directory's name will be the manga's name and the website from whit it is downloaded ex: One Piece_MD
/// the chapter folder's names will be ch-1_en, ch-2_en, etc... or similar
/// all of the informations will be stored in a toml file:
/// the toml file will contain :
/// the manga's name, its ID, the website from which it is downloaded (mangaDex, comick, etc...), the description, the tags, the status, the language(s) in which it  is downloaded
/// every downloaded chapter with its name, ID, TL group, chapter number, language, a key to find the corresponding and possibly more

pub struct DownloadData {
    pub manga_info: MangaInfo,
    /// the website that is downloaded from eg: mangaDex
    // pub source: Source,
    pub downloaded_language: &'static str,
    /// the date at which the manga is first downloaded
    // pub download_date: DateTime<Utc>,
    /// the date of the last download for the manga
    // pub update_date: DateTime<Utc>,
    pub low_quality_images: bool,
    /// the number of chapters that are skipped at the begining
    pub offset: i32,
}

const RESSOURCE_DIR: &'static str = ".md_light";

/// downloads the manga
/// the pages are stored in ~/.md_light/ (linux)
pub fn download_manga(download_infos: Option<DownloadData>) {
    // let os = check_os();
    let os = todo!();
    // Checks if the os is compatible first
    match os {
        "linux" | "Linux" => (),
        non_comp => {
            println!("Your OS ({non_comp}) is not compatible with the download feature yet. Please check later.");
            return;
        }
    }

    // let manga_dir_name = download_infos.manga_info.manga_name;
    use std::env;

    match env::home_dir() {
        Some(path) => println!("Your home directory, probably: {}", path.display()),
        None => println!("Impossible to get your home dir!"),
    }
}
