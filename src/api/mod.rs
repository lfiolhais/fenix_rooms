//! REST API
//!
//! The API has five handlers depending on the request. In `getters.rs` the
//! information received is parsed and proecessed.
extern crate pencil;

// ///////////////////////////////////////////////////////////
// Basic Structs
// ///////////////////////////////////////////////////////////
#[derive(Deserialize, Default)]
pub struct Room {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
    capacity: Capacity,
}

#[derive(Deserialize)]
pub struct GenericSpace {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
}

type Space = Vec<ContainedSpace>;

// ///////////////////////////////////////////////////////////
// Helper Structs
// ///////////////////////////////////////////////////////////
#[derive(Deserialize, Default)]
struct Capacity {
    normal: u64
}

#[derive(Deserialize)]
pub struct ContainedSpace {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
}

// ///////////////////////////////////////////////////////////
// Constants
// ///////////////////////////////////////////////////////////
const FENIX_BASE_URL: &'static str = "https://fenix.tecnico.ulisboa.pt/api/fenix/v1/spaces";
const DB_BASE_URL: &'static str = "http://0.0.0.0:32769"; //"https://asint-final-project.appspot.com";

// ///////////////////////////////////////////////////////////
// Modules
// ///////////////////////////////////////////////////////////
pub mod handlers;
mod getters;
