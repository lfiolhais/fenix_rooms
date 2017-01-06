//! REST API to interact with `FenixEDU` Spaces API
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
pub struct GenericSpace {
    name: String,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
    #[serde(default)]
    capacity: HashMap<String, u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContainedSpace {
    id: String,
    name: String,
}

type Space = Vec<ContainedSpace>;

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
mod misc {
    use api::pencil::UserError;
    use super::hyper::status::StatusCode;
    use super::getters;

    /// Checks in the FenixEDU API if the space with id `id` exists.
    ///
    /// # Arguments
    /// * `id` => id of the space.
    ///
    /// # Return Value
    /// If the room exists true, else false. If the `getters::get_spaces_from_id(<id>)`
    /// returns an error that will be the error passed.
    pub fn does_room_exist(id: &str) -> Result<bool, UserError> {
        // Get space with id `id` from FenixEDU
        match getters::get_spaces_from_id(id) {
            Ok(response) => {
                if response.status == StatusCode::Ok {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(err) => Err(err),
        }
    }
}
