//! Getters from the requests performed at Fenix
extern crate serde_json;

use super::pencil::UserError;
use super::{Space, Room, GenericSpace, ContainedSpace};
use super::FENIX_BASE_URL;
use utils;

/// Get all spaces at from Fenix
///
/// Send a GET request to the URL and read the data. Serialize the data into a
/// Space object and return a Result with the JSON string and the Space object.
///
/// # Output
/// Result of the transaction with a Space and String tuple and a `UserError`.
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

    Ok((space, get_response))
}

/// Get campus information
///
/// Get all spaces from Fenix. Search all spaces for the provided campus name.
/// If campus is found get its id and perform another GET request with the new
/// id. Read response serialize it and return the Result.
///
/// # Argument
/// * campus => Campus name to search for.
///
/// # Return Value
/// Result with the object and raw string. Error has a `UserError`.
pub fn get_campi(campus: &str) -> Result<(GenericSpace, String), UserError> {
    // Get all spaces
    let space: Space = match get_spaces() {
        Ok(space) => space.0,
        Err(err) => {
            return Err(err);
        }
    };

    let response: String = match search_contained_spaces(campus, &space) {
        Ok(response) => response,
        Err(err) => {
            return Err(err);
        }
    };

    let campus: GenericSpace = serde_json::from_str(&response).unwrap();

    Ok((campus, response))
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
/// * campus => Campus name to search for
/// * building => Building name to search for
///
/// # Return Value
/// Result with the object and raw string. Error has a `UserError`.
pub fn get_buildings(campus: &str, building: &str) -> Result<(GenericSpace, String), UserError> {
    // Get all spaces
    let campi: GenericSpace = match get_campi(campus) {
        Ok(campi) => campi.0,
        Err(err) => {
            return Err(err);
        }
    };

    let response: String = match search_contained_spaces(building, &campi.contained_spaces) {
        Ok(response) => response,
        Err(err) => {
            return Err(err);
        }
    };

    let building: GenericSpace = serde_json::from_str(&response).unwrap();

    Ok((building, response))
}

/// Get floor information
///
/// Get the specified floor from Fenix. Search all spaces for the provided
/// campus name. If campus is found get its id and perform another GET request
/// with the new id. Get all buildings from Fenix with the specified campus.
/// Search all buildings for the provided building name. Get all floors from
/// Fenix with the specified floor number. Search all floors for the provided
/// number. Read response serialize it and return the Result.
///
/// # Argument
/// * campus => Campus name to search for.
/// * building => Building name to search for
/// * floor => Floor number to search for
///
/// # Return Value
/// Result with the object and raw string. Error has a `UserError`.
pub fn get_floors(campus: &str,
                  building: &str,
                  floor: &str)
                  -> Result<(GenericSpace, String), UserError> {
    // Get all buildings
    let buildings: GenericSpace = match get_buildings(campus, building) {
        Ok(building) => building.0,
        Err(err) => {
            return Err(err);
        }
    };

    let response: String = match search_contained_spaces(floor, &buildings.contained_spaces) {
        Ok(response) => response,
        Err(err) => {
            return Err(err);
        }
    };

    let floor: GenericSpace = serde_json::from_str(&response).unwrap();

    Ok((floor, response))
}

/// Get floor information from another floor
///
/// From the floor information given by `get_floor()` search for another floor.
///
/// # Argument
/// * `parent_floor` => Parent floor that contains the floors to search for.
/// * `floor` => Floor number to search for
///
/// # Return Value
/// Result with the object and raw string. Error has a `UserError`.
pub fn get_floor_from_floor(parent_floor: &str,
                            floor: &str)
                            -> Result<(GenericSpace, String), UserError> {
    // Convert the string to object
    let parent_floor_obj: GenericSpace = serde_json::from_str(parent_floor).unwrap();

    let response: String = match search_contained_spaces(floor,
                                                         &parent_floor_obj.contained_spaces) {
        Ok(response) => response,
        Err(err) => {
            return Err(err);
        }
    };

    let floor: GenericSpace = serde_json::from_str(&response).unwrap();

    Ok((floor, response))
}

/// Get room information
///
/// From the number of arguments get the necessary information.
///
/// # Argument
/// * `args` => Vector that keeps all necessary arguments
///
/// # Return Value
/// Result with the object and raw string. Error has a `UserError`.
pub fn get_rooms(args: &[&str], name: &str) -> Result<String, UserError> {
    // Get the contained spaces inside each struct
    let contained_spaces: Vec<ContainedSpace> = match args.len() {
        2 => {
            match get_buildings(args[0], args[1]) {
                Ok(building) => building.0.contained_spaces,
                Err(err) => {
                    return Err(err);
                }
            }
        }
        3 => {
            match get_floors(args[0], args[1], args[2]) {
                Ok(floor) => floor.0.contained_spaces,
                Err(err) => {
                    return Err(err);
                }
            }
        }
        4 => {
            let floor_string = match get_floors(args[0], args[1], args[2]) {
                Ok(floor) => floor.1,
                Err(err) => {
                    return Err(err);
                }
            };

            match get_floor_from_floor(&floor_string, args[3]) {
                Ok(floor) => floor.0.contained_spaces,
                Err(err) => {
                    return Err(err);
                }
            }
        }
        _ => {
            let error = UserError::new("Error");
            return Err(error);
        }
    };

    let response: String = match search_contained_spaces(name, &contained_spaces) {
        Ok(response) => response,
        Err(err) => {
            return Err(err);
        }
    };

    let space: GenericSpace = serde_json::from_str(&response).unwrap();
    let room: Room = if space.contained_spaces.is_empty() {
        // It is a ROOM
        // OH HAI ROOM
        serde_json::from_str(&response).unwrap()
    } else {
        Default::default()
    };

    Ok(response)
}

/// Search for a space with a specified type and name
///
/// # Argument
/// * `type_name` => type of the space
/// * `name` => name of the space
/// * `contained_spaces` => spaces to search in
///
/// # Return Value:
/// String with the GET response or an `UserError`
fn search_contained_spaces(name: &str,
                           contained_spaces: &[ContainedSpace])
                           -> Result<String, UserError> {
    println!("Searching for: {}", name);
    let mut fenix_id: &str = "";
    for i in contained_spaces {
        println!("TEST: {}", i.name);
        if !i.name.is_empty() && utils::sanitize_string(&i.name) == name {
            fenix_id = &i.id;
            break;
        }
    }

    if fenix_id.is_empty() {
        let error = UserError::new(format!("No id found for space {}", name));
        return Err(error);
    }

    let url = &format!("{}/{}", FENIX_BASE_URL, fenix_id);

    let response = match utils::get_request(url) {
        Ok(response) => response,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    Ok(response)
}

/// Get all spaces at from Fenix
///
/// Send a GET request with the specified id.
///
/// # Output
/// Result of the transaction with a Space and String tuple and a `UserError`.
pub fn get_spaces_from_id(id: &str) -> Result<String, UserError> {
    // Format URL
    let url = &format!("{}/{}", FENIX_BASE_URL, id);

    // Send GET request to the url
    let get_response = match utils::get_request(url) {
        Ok(res) => res,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    Ok(get_response)
}
