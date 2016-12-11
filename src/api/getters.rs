//! Getters from the requests performed at Fenix
extern crate serde_json;

use super::pencil::UserError;
use super::{Space, Campus, Building, Floor, Room, ContainedSpace};
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
/// * campus => Campus name to search for.
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

    let response: String =
        match search_contained_spaces("CAMPUS", campus, &space) {
            Ok(response) => response,
            Err(err) => {
                return Err(err);
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
/// * campus => Campus name to search for
/// * building => Building name to search for
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

    let response: String =
        match search_contained_spaces("BUILDING", building, &campi.contained_spaces) {
            Ok(response) => response,
            Err(err) => {
                return Err(err);
            }
        };

    let building: Building = serde_json::from_str(&response).unwrap();

    return Ok((building, response));
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
/// Result with the object and raw string. Error has a UserError.
pub fn get_floors(campus: &str, building: &str, floor: &str) -> Result<(Floor, String), UserError> {
    // Get all buildings
    let buildings: Building = match get_buildings(campus, building) {
        Ok(building) => building.0,
        Err(err) => {
            return Err(err);
        }
    };

    let response: String =
        match search_contained_spaces("FLOOR", floor, &buildings.contained_spaces) {
            Ok(response) => response,
            Err(err) => {
                return Err(err);
            }
        };

    let floor: Floor = serde_json::from_str(&response).unwrap();

    return Ok((floor, response));
}

/// Get floor information from another floor
///
/// From the floor information given by `get_floor()` search for another floor.
///
/// # Argument
/// * parent_floor => Parent floor that contains the floors to search for.
/// * floor => Floor number to search for
///
/// # Return Value
/// Result with the object and raw string. Error has a UserError.
pub fn get_floor_from_floor(parent_floor: &String,
                            floor: &str)
                            -> Result<(Floor, String), UserError> {
    // Convert the string to object
    let parent_floor_obj: Floor = serde_json::from_str(&parent_floor).unwrap();

    let response: String =
        match search_contained_spaces("FLOOR", floor, &parent_floor_obj.contained_spaces) {
            Ok(response) => response,
            Err(err) => {
                return Err(err);
            }
        };

    let floor: Floor = serde_json::from_str(&response).unwrap();

    return Ok((floor, response));
}

/// Get room information
///
/// From the number of arguments get the necessary information.
///
/// # Argument
/// * args => Vector that keeps all necessary arguments
///
/// # Return Value
/// Result with the object and raw string. Error has a UserError.
pub fn get_rooms(args: &Vec<&str>) -> Result<(Room, String), UserError> {
    // Get the contained spaces inside each struct
    let contained_spaces: Vec<ContainedSpace> = match args.len() {
        2 => {
            match get_campi(args[0]) {
                Ok(campus) => campus.0.contained_spaces,
                Err(err) => {
                    return Err(err);
                }
            }
        }

        3 => {
            match get_buildings(args[0], args[2]) {
                Ok(building) => building.0.contained_spaces,
                Err(err) => {
                    return Err(err);
                }
            }
        }
        4 => {
            match get_floors(args[0], args[2], args[3]) {
                Ok(floor) => floor.0.contained_spaces,
                Err(err) => {
                    return Err(err);
                }
            }
        }
        5 => {
            let floor_string = match get_floors(args[0], args[2], args[3]) {
                Ok(floor) => floor.1,
                Err(err) => {
                    return Err(err);
                }
            };

            match get_floor_from_floor(&floor_string, args[4]) {
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

    let response: String = match search_contained_spaces("ROOM", args[1], &contained_spaces) {
        Ok(response) => response,
        Err(err) => {
            return Err(err);
        }
    };

    let room: Room = serde_json::from_str(&response).unwrap();

    return Ok((room, response));
}

/// Search for a space with a specified type and name
///
/// # Argument
/// * type_name => type of the space
/// * name => name of the space
/// * contained_spaces => spaces to search in
///
/// # Return Value:
/// String with the GET response or an UserError
fn search_contained_spaces(type_name: &str,
                           name: &str,
                           contained_spaces: &Vec<ContainedSpace>)
                           -> Result<String, UserError> {
    let mut fenix_id: &String = &format!("");
    for i in contained_spaces {
        if i.type_name == type_name && !i.name.is_empty() &&
           utils::sanitize_string(&i.name) == name {
            fenix_id = &i.id;
            break;
        }
    }

    let url = &format!("{}/{}", FENIX_BASE_URL, fenix_id);

    let response = match utils::get_request(url) {
        Ok(response) => response,
        Err(err) => {
            let error = UserError::new(err);
            return Err(error);
        }
    };

    return Ok(response);
}
