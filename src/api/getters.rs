//! Getters from the requests performed at Fenix
extern crate serde_json;

use super::pencil::UserError;
use super::{Space, Campus, Building, ContainedSpace};
use utils;

/// Get all spaces at IST
///
/// TODO: Description of the function
///
/// # Output
/// All spaces information with type, name and id.
pub fn get_spaces() -> Result<(Space, String), UserError> {

    // Send GET request to the url
    let get_response = match utils::get_request("https://fenix.tecnico.ulisboa.\
                                                 pt/api/fenix/v1/spaces") {
        Ok(res) => res,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    let space: Space = serde_json::from_str(&get_response).unwrap();

    println!("Space: {:#?}", space);

    return Ok((space, get_response));
}

/// TODO: Documentation
pub fn get_campi(campus: &str) -> Result<(Campus, String), UserError> {
    let space = match get_spaces() {
        Ok(space) => space.0,
        Err(err) => {
            return Err(err);
        }
    };

    // This needs to work for the other campus "Tecnológico e Nuclear"
    let mut fenix_campus_id: &String = &format!("");
    for c in &space {
        if c.get("name").unwrap().to_lowercase() == *campus {
            fenix_campus_id = c.get("id").unwrap();
            break;
        }
    }

    if fenix_campus_id.is_empty() {
        let error = UserError::new(format!("There was no campus found with name: {}", campus));
        return Err(error);
    }

    println!("The id found for {} is: {}", campus, fenix_campus_id);

    let url = &format!("https://fenix.tecnico.ulisboa.pt/api/fenix/v1/spaces/{}",
                       fenix_campus_id);

    let response = match utils::get_request(url) {
        Ok(response) => response,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    let building: Campus = serde_json::from_str(&response).unwrap();

    return Ok((building, response));
}
