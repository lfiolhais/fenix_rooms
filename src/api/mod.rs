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
    use utils::{from_json_to_obj, read_response_body};

    use super::hyper::status::StatusCode;
    use super::{getters, GenericSpace};

    /// Checks in the `FenixEDU` API if the space with id `id` exists. A space is
    /// considered a room when the parameter `contained_spaces` is empty.
    ///
    /// # Arguments
    /// * `id` => id of the space.
    ///
    /// # Return Value
    /// If the room exists true, else false. If the `getters::get_spaces_from_id(<id>)`
    /// returns an error that will be the error passed.
    pub fn is_room(id: &str) -> Result<bool, UserError> {
        // Get space with id `id` from FenixEDU
        match getters::get_spaces_from_id(id) {
            Ok(mut response) => {
                let json: String = match read_response_body(&mut response) {
                    Ok(body) => body,
                    Err(err) => {
                        return Err(UserError::new(err));
                    }
                };

                let is_room: bool = match from_json_to_obj::<GenericSpace>(&json) {
                    Ok(obj) => {
                        if obj.contained_spaces.is_empty() {
                            true
                        } else {
                            false
                        }
                    },
                    Err(err) => {
                        return Err(UserError::new(err));
                    }
                };

                if response.status == StatusCode::Ok && is_room {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Err(err) => Err(err),
        }
    }
}
