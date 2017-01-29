/// Client CLI for the Fenix Rooms web service
/// Author: Luís Fiolhais
///
///
extern crate fenix_rooms;
#[macro_use]
extern crate clap;
extern crate serde_json;
extern crate toml;

use fenix_rooms::utils::{delete_request, get_request, post_request, read_response_body};
use fenix_rooms::utils::{from_json_to_obj, from_obj_to_json};
use fenix_rooms::api::GenericSpace;

use clap::App;

use serde_json::Value;

use std::fs::File;
use std::io::{Write, Read, stdin};
use std::path::PathBuf;
use std::env::current_dir;
use std::process;

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
            // This is a safe unwrap because we created the file with the correct permissions
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

    let server_url: &str = match config.lookup("session.server_url") {
        Some(value) => value.as_str().unwrap(),
        None => panic!("Configuration file is malformed."),
    };

    // Warn user to login if the username or user_id is empty
    if !is_username || !is_user_id {
        println!("You need to login before proceeding.");
    }

    match matches.subcommand_name() {
        Some("login") => login(server_url),
        Some("check-in") => unimplemented!(),
        Some("check-out") => unimplemented!(),
        Some("rooms") => unimplemented!(),
        Some("spaces") => unimplemented!(),
        Some("id") => unimplemented!(),
        Some("path") => unimplemented!(),
        Some("create_room") => unimplemented!(),
        _ => panic!("You need to provide a subcommand"),
    };

    process::exit(0);
}

fn login(server_url: &str) {
    let mut input: String = String::new();
    println!("Username: ");
    stdin().read_line(&mut input).expect("Failed reading from stdin");

    let url: String = format!("{}/api/create_user", server_url);
    let body: String = format!("{{ \"username\": \"{}\" }}", input.trim());

    match post_request(&url, &body) {
        Ok(mut response) => {
            let body: String = match read_response_body(&mut response) {
                Ok(body) => body,
                Err(err) => panic!("Failed reading response body: {}", err),
            };

            let obj: Value = match serde_json::from_str(&body) {
                Ok(obj) => obj,
                Err(err) => panic!("Failed to parse JSON: {}", err),
            };
            println!("JSON: {:#?}", obj);

            // if response.status == StatusCode::Ok {

            // } else if response.status == StatusCode::Conflict {

            // } else {
            //     println!("Something went wrong in the server or the database.");
            // }
        }
        Err(err) => panic!("The post request failed with: {}", err),
    };
}
