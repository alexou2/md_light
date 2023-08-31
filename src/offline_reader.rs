use crate::md_struct::*;
use crate::online_md;
use reqwest;
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
