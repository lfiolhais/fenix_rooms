extern crate pencil;

use std::collections::HashMap;

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
    descritpion: String,
    capacity: Capacity,
    events: Event,
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

type Space = Vec<TopLevelSpace>;

// Helper Structs
#[derive(Deserialize)]
struct Period {
    start: String,
    end: String,
}

#[derive(Deserialize)]
struct Event {
    #[serde(rename="type")]
    type_name: String,
    start: String,
    end: String,
    weekday: String,
    day: String,
    period: Period,
    description: String,
    title: String,
}

#[derive(Deserialize)]
struct Capacity {
    normal: String,
    exam: String,
}

#[derive(Deserialize)]
pub struct TopLevelSpace {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct ContainedSpace {
    #[serde(rename="type")]
    type_name: String,
    id: String,
    name: String,
    #[serde(rename="topLevelSpace")]
    top_level_space: TopLevelSpace,
}

type ParentSpace = TopLevelSpace;

// Constants
const FENIX_BASE_URL: &'static str = "https://fenix.tecnico.ulisboa.pt/api/fenix/v1/spaces";

// Modules
pub mod handlers;
mod getters;
