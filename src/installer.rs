use crate::cli_options::CliArgs;
use colored::Colorize;
use std::fs::{create_dir, remove_dir, remove_file, write};
use std::io::Write;
use std::path::PathBuf;

/// initialises the server by creating the required files for the server
pub fn init(args: &mut CliArgs) {
    args.command = None;
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

pub fn uninstall() {
    let mut config_path = dirs::config_local_dir().unwrap();
    config_path.push("md_light");
    config_path.push("mdl.conf");

    if config_path.exists() {
        println!("Path to remove: {}", config_path.to_str().unwrap().red());
        let prompt = prompt_true_false(
            "Do you want to remove your md_light configuration Y/[n] ",
            false,
        );
        println!("{}", prompt);
    }

    std::process::exit(0)
}

fn delete_file_or_dir(name: PathBuf) {
    todo!()
}

fn prompt_true_false(name: &str, default: bool) -> bool {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    let resp = line.trim();

    match resp {
        "Y" | "y" | "yes" | "Yes" => true,
        "N" | "n" | "no" | "No" | "" => false,
        _ => default,
    }
}
