use crate::cli_options::CliArgs;
use dirs;

use std::fs::{create_dir, write};
use std::path::PathBuf;

use toml;

// base url of the repo

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
    write(file_path, config_file).unwrap();
}

fn init_config_file(args: &CliArgs) -> String {
    let config_content = toml::to_string(args).unwrap();
    println!("{}", config_content);
    config_content
}

fn download_ressources(){
    let cache_dir = dirs::cache_dir().unwrap();

    let template_dir = cache_dir.clone().push("templates");
    let ressources_dir = cache_dir.clone().push("ressources");


}
