//! REST API to interact with `FenixEDU` Spaces API
//!
//! The API has five handlers depending on the request. In `getters.rs` the
//! information received is parsed and proecessed.
extern crate pencil;
extern crate hyper;

// ///////////////////////////////////////////////////////////
// Basic Structs
// ///////////////////////////////////////////////////////////
#[derive(Deserialize, Serialize, Default)]
pub struct GenericSpace {
    name: String,
    #[serde(rename="containedSpaces")]
    contained_spaces: Vec<ContainedSpace>,
    #[serde(skip_serializing_if="Option::is_none")]
    capacity: Option<Capacity>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContainedSpace {
    id: String,
    name: String,
}

#[derive(Default, Deserialize, Serialize)]
pub struct Capacity {
    normal: u64,
}

type Space = Vec<ContainedSpace>;

// ///////////////////////////////////////////////////////////
// Enumerators
// ///////////////////////////////////////////////////////////
/// Possible search results when searching for a name in the contained spaces parameter
///
/// # Values
/// * `Ok` is used when the desired search result was achieved;
/// * `NotFound` is used when the result isn't found;
/// * `Error` is used when an error is found that is out of the context of this
/// application, e.g., a service crashing.
pub enum SearchResult {
    Ok(String),
    NotFound(String),
    Error(String),
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
mod misc {
    use api::pencil::{Response as PencilResponse, UserError};
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
                    Ok(obj) => obj.contained_spaces.is_empty(),
                    Err(_) => false,
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

    /// Build a Response from the provided message and status code
    ///
    /// # Arguments
    /// * `status_code` => the status code of the response
    /// * `msg` => message to be sent to the response
    ///
    /// # Return Value
    /// The response built with the specified parameters
    pub fn build_response(status_code: u16, msg: &str) -> PencilResponse {
        // Build response and set content to JSON response
        let mut response = PencilResponse::from(msg);
        response.set_content_type("application/json");
        response.status_code = status_code;

        response
    }
}
