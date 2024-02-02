use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;



lazy_static! {
    /// the startup arguments for the server
   pub static ref CONFIG: CliArgs = get_startup_config();
}

/// loads the config file if the user used --conf
fn get_startup_config() -> CliArgs {
    let mut args = CliArgs::parse();
    
    if args.recommended {
        args.lan = true;
        args.secure = true;
    }

    if args.config {
        println!("Reading the config file...");
        let mut file_path = dirs::config_local_dir().unwrap();
        file_path.push("md_light");
        file_path.push("mdl.conf");

        let config_file = std::fs::read_to_string(file_path);
        match config_file {
            Ok(e) => {
                args = parse_config_file(e);
                println!("reading the file: OK");
            }
            Err(_) => println!("Unable to load the config file. Starting with the other arguments"),
        }
    }
    return args;
}
/// parses the config file 
fn parse_config_file(content: String) -> CliArgs {
    let config = toml::from_str(&content).unwrap();
    config
}



/// A web server that uses the mangadex api with a lighweight frontend for potato devices
#[derive(Parser, Serialize, Debug, Deserialize)]
#[command(propagate_version = true)]
#[command(author = "_alexou_", version = "0.1.2", about , long_about = None, name = "completion-derive")]
pub struct CliArgs {
    /// Downloads the ressources files for the front-end
    #[arg(short, long)]
    pub install: bool,

    /// Allows other lan devices to connect to the server (you will need to open the port on your device)
    #[arg(short, long)]
    pub lan: bool,

    /// Uses the lower quality images from mangadex instead of the high quality ones
    #[arg(short, long)]
    pub datasaver: bool,

    /// Restricts functionnalities for non-admin users
    #[arg(short, long)]
    pub secure: bool,

    /// Manually set the port for the server
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,

    /// Uses the recommended server options
    #[arg(short, long)]
    pub recommended: bool,

    /// uses the config file to start the server
    #[arg(short, long = "config")]
    pub config: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug, Serialize, Deserialize, Clone)]
pub enum Commands {
    /// Creates the config and service files for the server. The other parameters used will also be the default params for the config file
    Init,
}
