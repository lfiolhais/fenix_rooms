extern crate fenix_rooms;
#[macro_use]
extern crate clap;
extern crate toml;

use clap::App;

use fenix_rooms::utils::{delete_request, get_request, post_request, read_response_body};
use fenix_rooms::api::GenericSpace;

const SERVER_URL: &'static = "https://fenix-rooms.herokuapp.com";

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
}
