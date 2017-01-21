/// Client CLI for the Fenix Rooms web service
/// Author: Luís Fiolhais
///
///
extern crate fenix_rooms;
#[macro_use]
extern crate clap;
extern crate toml;

use clap::App;

use fenix_rooms::utils::{delete_request, get_request, post_request, read_response_body};
use fenix_rooms::api::GenericSpace;

use std::fs::File;
use std::io::Write;
use std::env::current_dir;

const DEFAULT_CONFIG_TOML: &'static str =
    "[session]
server_url = \"https://fenix-rooms.herokuapp.com\"
user_id = \"\"
username = \"\"
";

fn main() {
    // Build CLI parser
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Try to read config file
    let mut file: File = match File::open("fenix-rooms-user.toml") {
        Ok(file) => file,
        // Assume that if the open file fails the file doesn't exist.
        // Create file and write defaults
        Err(_) => {
            println!("Creating configuration default at {:#?}",
                     current_dir().unwrap());
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
            new_file
        }
    };

}
