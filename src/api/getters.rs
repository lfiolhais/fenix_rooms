//! Getters from the requests performed at Fenix
extern crate serde_json;

use super::pencil::UserError;
use super::{Space, Campus, Building, ContainedSpace};
use utils;

/// Get all spaces at from Fenix
///
/// Send a GET request to the URL and read the data. Serialize the data into a
/// Space object and return a Result with the JSON string and the Space object.
///
/// # Output
/// Result of the transaction with a Space and String tuple and a UserError.
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

/// Get campus information
///
/// Get all spaces from Fenix. Search all spaces for the provided campus name.
/// If campus is found get its id and perform another GET request with the new
/// id. Read response serialize it and return the Result.
///
/// # Argument
/// * campus => Campus to search for.
///
/// # Return Value
/// Result with the object and raw string. Error has a UserError.
pub fn get_campi(campus: &str) -> Result<(Campus, String), UserError> {
    // Get all spaces
    let space: Space = match get_spaces() {
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
