//! Handlers for the REST API
//!
//! Each handler reads the request if need be, gets the information from getters
//! and returns a Response accordingly,
extern crate serde_json;

use super::pencil::{Request, Response, PencilResult};
use super::pencil::{UserError, PenUserError};
use super::GenericSpace;
use super::getters;
use utils;
use super::DB_BASE_URL;

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
    let space: String = match getters::get_spaces_from_id("") {
        Ok(space) => space,
        Err(err) => {
            return Err(PenUserError(err));
        }
    };

    // Build response and set content to JSON
    let mut response = Response::from(space);
    response.set_content_type("application/json");

    Ok(response)
}

/// Handler for IDs using the `FenixEDU` API
///
/// # Return Value
/// Error if the `get_spaces_from_id()` fails. Otherwise read the contents and
/// send it as JSON.
pub fn id_handler(request: &mut Request) -> PencilResult {
    // Get ID from request
    let id: &str = match request.view_args.get("id") {
        Some(id) => id as &str,
        None => "",
    };

    let generic_space: String = match getters::get_spaces_from_id(id) {
        Ok(data) => data,
        Err(err) => {
            return Err(PenUserError(err));
        }
    };

    let space: GenericSpace = serde_json::from_str(&generic_space).unwrap();

    let mut response: Response;
    if !space.contained_spaces.is_empty() {
        let generic_space_contained_spaces_serialized: String =
            serde_json::to_string(&space.contained_spaces).unwrap();

        // Build response and set content to JSON
        response = Response::from(generic_space_contained_spaces_serialized);
        response.set_content_type("application/json");
    } else {
        // Build response and set content to JSON
        response = Response::from(generic_space);
        response.set_content_type("application/json");
    }

    Ok(response)
}

/// Creates a User in the Database
///
/// Create a user in the database with the specified username in the body. If
/// multiple usernames are provided only the first will be considered.
///
/// # Arguments
/// * request - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
pub fn create_user_handler(request: &mut Request) -> PencilResult {
    // Get the username from the body of the request if it exists
    let username: &str = match request.form().get("username") {
        Some(username) => username,
        None => "",
    };

    let status_code: u16;
    let mut buffer: String = "".to_owned();

    if username.is_empty() {
        status_code = 204;
    } else {
        let url: &str = &format!("{}/users", DB_BASE_URL);
        let body: &str = &format!("{{\"username\": \"{}\"}}", username);

        println!("Body: {}", body);

        let mut response = match utils::post_request(url, body) {
            Ok(response) => response,
            Err(err) => {
                let error = UserError::new(err);
                return Err(PenUserError(error));
            }
        };
        buffer = match utils::read_response_body(&mut response) {
            Ok(buffer) => buffer,
            Err(err) => {
                return Err(PenUserError(UserError::new(err)));
            }
        };
        status_code = 200;
    }

    let mut response = Response::from(buffer);
    response.set_content_type("application/json");
    response.status_code = status_code;

    Ok(response)
}

/// Creates a Room in the Database
///
/// Create a room in the database with the specified id, capacity, location and
/// admin_id in the body. Only the admin can create rooms in the DB.
///
/// # Arguments
/// * request - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
pub fn create_room_handler(request: &mut Request) -> PencilResult {
    // Get the id and capacity from the body of the request if they exists. I
    // need to clone the parameter because Pencil returns a reference to the
    // Struct and doesn't allow me consume the contents of the form. We are
    // essentially wasting memory.
    let location: String = match request.form().get("location") {
        Some(location) => location.clone(),
        None => "".to_owned(),
    };

    let fenix_id: String = match request.form().get("fenix_id") {
        Some(fenix_id) => fenix_id.clone(),
        None => "".to_owned(),
    };

    let capacity: String = match request.form().get("capacity") {
        Some(capacity) => capacity.clone(),
        None => "".to_owned(),
    };

    let admin_id: String = match request.form().get("admin_id") {
        Some(admin_id) => admin_id.to_owned(),
        None => "".to_owned(),
    };

    let status_code: u16;
    let mut buffer: String = "".to_owned();

    if admin_id == "0" {
        if location.is_empty() || capacity.is_empty() || fenix_id.is_empty() {
            status_code = 204;
        } else {
            let url: &str = &format!("{}/rooms", DB_BASE_URL);
            let body: &str = &format!("{{\"location\": \"{}\", \"capacity\": {}, \"fenix_id\": {}}}",
                                      location,
                                      capacity,
                                      fenix_id);

            println!("Body: {}", body);

            let mut response = match utils::post_request(url, body) {
                Ok(response) => response,
                Err(err) => {
                    let error = UserError::new(err);
                    return Err(PenUserError(error));
                }
            };
            buffer = match utils::read_response_body(&mut response) {
                Ok(buffer) => buffer,
                Err(err) => {
                    return Err(PenUserError(UserError::new(err)));
                }
            };
            status_code = 200;
        }

        let mut response = Response::from(buffer);
        response.set_content_type("application/json");
        response.status_code = status_code;

        return Ok(response);
    }

    let mut response = Response::from("{ \"error\": \"Unauthorized access to DB\"}");
    response.set_content_type("application/json");
    Ok(response)
}

/// Checks in in the Database
///
/// The check in is performed with a room id and a user id. Then, a POST request
/// is sent and its content read and sent to the client.
///
/// # Arguments
/// * request - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
pub fn check_in_handler(request: &mut Request) -> PencilResult {
    let user_id: String = match request.form().get("user_id") {
        Some(user_id) => user_id.clone(),
        None => "".to_owned(),
    };

    let room_id: String = match request.form().get("room_id") {
        Some(room_id) => room_id.clone(),
        None => "".to_owned(),
    };

    let status_code: u16;
    let mut buffer: String = "".to_owned();

    if room_id.is_empty() || user_id.is_empty() {
        status_code = 204;
    } else {
        let url: &str = &format!("{}/checkins", DB_BASE_URL);
        let body: &str = &format!("{{\"user_id\": {}, \"room_id\": {}}}", user_id, room_id);

        println!("Body: {}", body);

        let mut response = match utils::post_request(url, body) {
            Ok(response) => response,
            Err(err) => {
                let error = UserError::new(err);
                return Err(PenUserError(error));
            }
        };
        buffer = match utils::read_response_body(&mut response) {
            Ok(buffer) => buffer,
            Err(err) => {
                return Err(PenUserError(UserError::new(err)));
            }
        };
        status_code = 200;
    }

    let mut response = Response::from(buffer);
    response.set_content_type("application/json");
    response.status_code = status_code;

    Ok(response)
}

/// Checks out in the Database
///
/// The check out is performed with a room id and a user id. Then, a POST request
/// is sent and its content read and sent to the client.
///
/// # Arguments
/// * request - The request sent by the client
///
/// # Output
/// A Response with a JSON messsage and correct status code.
pub fn check_out_handler(request: &mut Request) -> PencilResult {
    let user_id: String = match request.form().get("user_id") {
        Some(user_id) => user_id.clone(),
        None => "".to_owned(),
    };

    let room_id: String = match request.form().get("room_id") {
        Some(room_id) => room_id.clone(),
        None => "".to_owned(),
    };

    let status_code: u16;
    let mut buffer: String = "".to_owned();

    if room_id.is_empty() || user_id.is_empty() {
        status_code = 204;
    } else {
        let url: &str = &format!("{}/checkins", DB_BASE_URL);
        let body: &str = &format!("{{\"user_id\": {}, \"room_id\": {}}}", user_id, room_id);

        println!("Body: {}", body);

        let mut response = match utils::post_request(url, body) {
            Ok(response) => response,
            Err(err) => {
                let error = UserError::new(err);
                return Err(PenUserError(error));
            }
        };
        buffer = match utils::read_response_body(&mut response) {
            Ok(buffer) => buffer,
            Err(err) => {
                return Err(PenUserError(UserError::new(err)));
            }
        };
        status_code = 201;
    }

    let mut response = Response::from(buffer);
    response.set_content_type("application/json");
    response.status_code = status_code;

    Ok(response)
}
