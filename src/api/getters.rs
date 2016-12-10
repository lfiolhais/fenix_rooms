//! Getters from the requests performed at Fenix
extern crate serde_json;

use super::pencil::UserError;
use super::{Space, Campus, Building, Floor, Room};
use super::FENIX_BASE_URL;
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
    let get_response = match utils::get_request(FENIX_BASE_URL) {
        Ok(res) => res,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    let space: Space = serde_json::from_str(&get_response).unwrap();

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

    let mut fenix_campus_id: &String = &format!("");
    for c in &space {
        let std_string: String = utils::sanitize_string(&c.name);
        if std_string == campus {
            fenix_campus_id = &c.id;
            break;
        }
    }

    if fenix_campus_id.is_empty() {
        let error = UserError::new(format!("There was no campus found with name: {}", campus));
        return Err(error);
    }

    println!("The id found for {} is: {}", campus, fenix_campus_id);

    let url = &format!("{}/{}", FENIX_BASE_URL, fenix_campus_id);

    let response = match utils::get_request(url) {
        Ok(response) => response,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    let campus: Campus = serde_json::from_str(&response).unwrap();

    return Ok((campus, response));
}

/// Get building information
///
/// Get all spaces from Fenix. Search all spaces for the provided campus name.
/// If campus is found get its id and perform another GET request with the new
/// id. Get all buildings from Fenix with the specified campus. Search all
/// buildings for the provided building name. Read response serialize it and
/// return the Result.
///
/// # Argument
/// * campus => Campus to search for.
///
/// # Return Value
/// Result with the object and raw string. Error has a UserError.
pub fn get_buildings(campus: &str, building: &str) -> Result<(Building, String), UserError> {
    // Get all spaces
    let campi: Campus = match get_campi(campus) {
        Ok(campi) => campi.0,
        Err(err) => {
            return Err(err);
        }
    };

    let mut fenix_building_id: &String = &format!("");
    for c in &campi.contained_spaces {
        let std_string: String = utils::sanitize_string(&c.name);
        println!("Test String: {}", std_string);
        if std_string == building {
            fenix_building_id = &c.id;
            break;
        }
    }

    if fenix_building_id.is_empty() {
        let error = UserError::new(format!("There was no building found at {} with name: {}",
                                           campus,
                                           building));
        return Err(error);
    }

    println!("The id found for {} at {} is: {}",
             building,
             campus,
             fenix_building_id);

    let url = &format!("{}/{}", FENIX_BASE_URL, fenix_building_id);

    let response = match utils::get_request(url) {
        Ok(response) => response,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    println!("TESTE");
    println!("Response: {:?}", response);
    let building: Building = serde_json::from_str(&response).unwrap();

    println!("TESTE");
    return Ok((building, response));
}

/// Get building information
///
/// Get all spaces from Fenix. Search all spaces for the provided campus name.
/// If campus is found get its id and perform another GET request with the new
/// id. Get all buildings from Fenix with the specified campus. Search all
/// buildings for the provided building name. Read response serialize it and
/// return the Result.
///
/// # Argument
/// * campus => Campus to search for.
///
/// # Return Value
/// Result with the object and raw string. Error has a UserError.
pub fn get_floors(campus: &str, building: &str, floor: &str) -> Result<(Floor, String), UserError> {
    // Get all buildings
    let buildings: Building = match get_buildings(campus, building) {
        Ok(building) => building.0,
        Err(err) => {
            return Err(err);
        }
    };

    let mut fenix_floor_id: &String = &format!("");
    for c in &buildings.contained_spaces {
        let std_string: String = utils::sanitize_string(&c.name);
        println!("Test String: {}", std_string);
        if std_string == floor {
            fenix_floor_id = &c.id;
            break;
        }
    }

    if fenix_floor_id.is_empty() {
        let error = UserError::new(format!("There was no floor found in {} at {} with name: {}",
                                           building,
                                           campus,
                                           floor));
        return Err(error);
    }

    println!("The id found for {} at {} is: {}",
             building,
             campus,
             fenix_floor_id);

    let url = &format!("{}/{}", FENIX_BASE_URL, fenix_floor_id);

    let response = match utils::get_request(url) {
        Ok(response) => response,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    let floor: Floor = serde_json::from_str(&response).unwrap();

    return Ok((floor, response));
}
