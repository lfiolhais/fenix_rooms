//! Handlers for the REST API
//!
//! Each handler reads the request if need be, gets the information from getters
//! and returns a Response accordingly,
use super::pencil::{Request, Response, PencilResult};
use super::pencil::{UserError, PenUserError};
use super::getters;

/// Handler for all spaces at IST
///
/// The handler calls `get_spaces()` to perform the GET request required. If the
/// request was successful its contents will be sent as JSON. Otherwise an error
/// will be sent, provided by the function.
///
/// # Arguments
/// * _ => The Pencil framework requires the handler signature to be
///        of the type `fn(&mut Request) -> PencilResult`. However, this handler
///        doesn't require access to the Request sent by the client. So we
///        declare as unused with `_`.
///
/// # Return Value
/// Error if the `get_campi()` fails. Otherwise read the contents and send it as
/// JSON.
pub fn spaces_handler(_: &mut Request) -> PencilResult {
    // Get all spaces from Fenix
    let space: String = match getters::get_spaces() {
        Ok(space) => space.1,
        Err(err) => {
            return Err(PenUserError(err));
        }
    };

    // Build response and set content to JSON
    let mut response = Response::from(space);
    response.set_content_type("application/json");

    return Ok(response);
}

/// Handler for a campus at IST
///
/// The handler calls `get_campi(campus)` to perform the GET request required
/// and search the contents for a match. If the request was successful its
/// contents will be sent as JSON. Otherwise an error will be sent, provided by
/// the function.
///
/// # Return Value
/// Error if somehow the campus field is empty and the getter
/// errors. Otherwise JSON contents are sent.
pub fn campus_handler(request: &mut Request) -> PencilResult {
    // Get Campus
    let my_campus: &str = match request.view_args.get("campus") {
        Some(my_campus) => my_campus as &str,
        None => {
            let error = UserError::new("The campus field is empty");
            return Err(PenUserError(error));
        }
    };

    let campus: String = match getters::get_campi(my_campus) {
        Ok(campus) => campus.1,
        Err(err) => {
            return Err(PenUserError(err));
        }
    };

    // Build response and set content to JSON
    let mut response = Response::from(campus);
    response.set_content_type("application/json");

    return Ok(response);
}

/// Handler for a building at IST
///
/// The handler calls `get_building(building, campus)` to perform the GET
/// request required and search the contents for a match. If the request was
/// successful its contents will be sent as JSON. Otherwise an error will be
/// sent, provided by the function.
///
/// # Return Value
/// Error if somehow the campus field is empty and the getter
/// errors. Otherwise JSON contents are sent.
pub fn building_handler(request: &mut Request) -> PencilResult {
    // Get Campus
    let my_campus: &str = match request.view_args.get("campus") {
        Some(my_campus) => my_campus as &str,
        None => {
            let error = UserError::new("The campus field is empty");
            return Err(PenUserError(error));
        }
    };

    // Get Building
    let my_building: &str = match request.view_args.get("building") {
        Some(my_building) => my_building as &str,
        None => {
            let error = UserError::new("The building field is empty");
            return Err(PenUserError(error));
        }
    };

    let building: String = match getters::get_buildings(my_campus, my_building) {
        Ok(building) => building.1,
        Err(err) => {
            return Err(PenUserError(err));
        }
    };

    // Build response and set content to JSON
    let mut response = Response::from(building);
    response.set_content_type("application/json");

    return Ok(response);
}

pub fn floor_handler(request: &mut Request) -> PencilResult {
    // Get Campus
    let my_campus: &str = match request.view_args.get("campus") {
        Some(my_campus) => my_campus as &str,
        None => {
            let error = UserError::new("The campus field is empty");
            return Err(PenUserError(error));
        }
    };

    // Get Building
    let my_building: &str = match request.view_args.get("building") {
        Some(my_building) => my_building as &str,
        None => {
            let error = UserError::new("The building field is empty");
            return Err(PenUserError(error));
        }
    };

    // Get Floor
    let my_floor: &str = match request.view_args.get("floor") {
        Some(my_floor) => my_floor as &str,
        None => {
            let error = UserError::new("The floor field is empty");
            return Err(PenUserError(error));
        }
    };

    let floor: String = match getters::get_floors(my_campus, my_building, my_floor) {
        Ok(floor) => floor.1,
        Err(err) => {
            return Err(PenUserError(err));
        }
    };

    // Build response and set content to JSON
    let mut response = Response::from(floor);
    response.set_content_type("application/json");

    return Ok(response);
}
