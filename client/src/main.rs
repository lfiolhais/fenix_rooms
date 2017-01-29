/// Client CLI for the Fenix Rooms web service
/// Author: Luís Fiolhais
///
///
extern crate fenix_rooms;
#[macro_use]
extern crate clap;
extern crate toml;

use fenix_rooms::utils::{delete_request, get_request, post_request, read_response_body};
use fenix_rooms::api::GenericSpace;

use clap::App;

use std::fs::File;
use std::io::{Write, Read};
use std::path::PathBuf;
use std::env::current_dir;

const DEFAULT_CONFIG_TOML: &'static str = "[session]
server_url = \"https://fenix-rooms.herokuapp.com\"
user_id = \"\"
username = \"\"
";

fn main() {
    // Build CLI parser
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut config_location: PathBuf = current_dir().unwrap();
    config_location.push("fenix-rooms-user.toml");

    // Try to read config file
    let mut file: File = match File::open("fenix-rooms-user.toml") {
        Ok(file) => file,
        // Assume that if the open file fails the file doesn't exist.
        // Create file and write defaults
        Err(_) => {
            println!("Creating configuration default at {:#?}", config_location);
            let mut new_file: File = match File::create("fenix-rooms-user.toml") {
                Ok(new_file) => new_file,
                Err(err) => panic!("Failed to create file fenix-rooms-user.toml with: {}", err),
            };
            match new_file.write_all(DEFAULT_CONFIG_TOML.as_bytes()) {
                Ok(_) => (),
                Err(err) => {
                    panic!("Failed to write the contents of the default file with: {}",
                           err)
                }
            };
            File::open("fenix-rooms-user.toml").unwrap()
        }
    };

    // Read File
    let mut read_file: String = "".to_owned();
    match file.read_to_string(&mut read_file) {
        Ok(_) => {}
        Err(err) => panic!("Failed to read configuration file with: {:#?}", err),
    };

    // Transform file into object
    let config: toml::Value = read_file.parse().unwrap();

    let is_username: bool = match config.lookup("session.username") {
        Some(value) => !value.as_str().unwrap().is_empty(),
        None => panic!("Configuration file is malformed."),
    };

    let is_user_id: bool = match config.lookup("session.user_id") {
        Some(value) => !value.as_str().unwrap().is_empty(),
        None => panic!("Configuration file is malformed."),
    };

    // Warn user to login if the username or user_id is empty
    if !is_username || !is_user_id {
        println!("You need to login before proceeding.");
    }

    if matches.is_present("login") {

    }
}
