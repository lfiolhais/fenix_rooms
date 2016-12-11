extern crate pencil;

// Basic Structs
#[derive(Deserialize)]
pub struct Room {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="topLevelSpace")]
    top_level_space: TopLevelSpace,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
    #[serde(rename="parentSpace")]
    parent_space: ParentSpace,
    description: String,
    capacity: Capacity,
}

#[derive(Deserialize)]
pub struct Floor {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="topLevelSpace")]
    top_level_space: TopLevelSpace,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
    #[serde(rename="parentSpace")]
    parent_space: ParentSpace,
}

#[derive(Deserialize)]
pub struct Building {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="topLevelSpace")]
    top_level_space: TopLevelSpace,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
    #[serde(rename="parentSpace")]
    parent_space: ParentSpace,
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

type Space = Vec<ContainedSpace>;

// Helper Structs
#[derive(Deserialize)]
struct Period {
    start: String,
    end: String,
}

#[derive(Deserialize)]
struct Capacity {
    normal: u64,
    exam: u64,
}

#[derive(Deserialize)]
struct TopLevelSpace {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
}

#[derive(Deserialize)]
pub struct ContainedSpace {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String, /* #[serde(rename="topLevelSpace")]
                   * top_level_space: TopLevelSpace, */
}

type ParentSpace = TopLevelSpace;

// Constants
const FENIX_BASE_URL: &'static str = "https://fenix.tecnico.ulisboa.pt/api/fenix/v1/spaces";

// Modules
pub mod handlers;
mod getters;
