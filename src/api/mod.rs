//! REST API
//!
//! The API has five handlers depending on the request. In `getters.rs` the
//! information received is parsed and proecessed.
extern crate pencil;
extern crate hyper;

use std::collections::HashMap;

// ///////////////////////////////////////////////////////////
// Basic Structs
// ///////////////////////////////////////////////////////////
#[derive(Deserialize, Serialize, Default)]
pub struct Room {
    name: String,
    capacity: HashMap<String, u64>,
}

#[derive(Deserialize, Serialize)]
pub struct GenericSpace {
    name: String,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
}

#[derive(Deserialize, Serialize)]
pub struct ContainedSpace {
    id: String,
    name: String,
}

// ///////////////////////////////////////////////////////////
// Constants
// ///////////////////////////////////////////////////////////
const FENIX_BASE_URL: &'static str = "https://fenix.tecnico.ulisboa.pt/api/fenix/v1/spaces";
const DB_BASE_URL: &'static str = "https://asint-project.herokuapp.com";

// ///////////////////////////////////////////////////////////
// Modules
// ///////////////////////////////////////////////////////////
pub mod handlers;
mod getters;
