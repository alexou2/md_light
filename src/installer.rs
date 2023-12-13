use dirs::home_dir;
use reqwest;
use std::env::consts::OS;
use std::fs;
use std::fs::{create_dir, read_dir, write};
use std::io::Write;
use std::os;
use std::ptr::null;

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

fn prompt(prompt: &str) -> String {
    let mut line = String::new();
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: Could not read a line");

    return line.trim().to_owned();
}

// creates a systemd service for running the program on a server
pub fn create_service(mut path_to_binary: &str) {
    match OS {
        "linux" => (),
        "windows" => {
            println!("Get linux, you pleb");
            std::process::exit(1)
        }
        _ => {
            println!(
                r"You need Linux to use this functionnality.
        If you think you os supports systemd services, please create an issue on GitLab."
            );
            std::process::exit(1)
        }
    }

    let contents = fs::read(&path_to_binary);
    match contents {
        Ok(_) => {
            println!("the file exists")
        }
        Err(err) => {
            println!("file not found : {err}");
            std::process::exit(1);
        }
    }

    let contents = fs::read_dir("/etc/systemd/system/");
    match contents {
        Ok(_) => println!("Your OS supports systemd services"),
        Err(err) => {
            println!("Directory not found : {err}");
            std::process::exit(1);
        }
    }

    let global_dir =
        prompt("Enter the path to the working directory folder [default /home/<user>/] > ");
    let ressources_dir = fs::read_dir(format!("{}/ressources", &global_dir));
    match ressources_dir {
        Ok(_) => (),
        Err(err) => {
            println!("Directory not found : {err}");
            std::process::exit(1);
        }
    }

    let args = prompt("Startup args (use -r for reccomanded configuration) > ");

    let file_content = format!(
        r"[Unit]
    Description=An mangaDex server with a light frontend
    After=network.target
    
    [Service]
    Type=simple
    ExecStart={path_to_binary} {args}
    WorkingDirectory={}
    
    [Install]
    WantedBy=default.target",
        global_dir
    );

    // lets the user confirm before creating the file
    let confirm_write = prompt("write file in /etc/systemd/system/md_light.service ? [Y/n]");
    match confirm_write.as_str() {
        "" | " " | "y" | "Y" => {
            fs::write("/etc/systemd/system/md_light.service", &file_content)
                .expect("Unable to create file");
            println!("successfully created file");
        }
        "n" => (),
        _ => {
            println!("\nAbort");
            std::process::exit(1)
        }
    }
    println!(
        r"How to manage the server: 
    start the service once: systemctl start md_light.service 
    stop the service once: systemctl stop md_light.service
    
    start the service at boot: systemctl enable md_light.service
    stop the service from starting on boot: systemctl disable md_light.service

    View status and logs: systemctl status md_light.service
    "
    );
}
