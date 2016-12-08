extern crate pencil;

use std::collections::HashMap;

#[derive(Deserialize)]
struct ContainedSpace {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="topLevelSpace")]
    top_level_space: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Building {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="topLevelSpace")]
    top_level_space: HashMap<String, String>,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
    #[serde(rename="parentSpace")]
    parent_space: HashMap<String, String>,
}

#[derive(Deserialize)]
pub struct Campus {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
}

type Space = Vec<HashMap<String, String>>;

pub mod handlers;
mod getters;
